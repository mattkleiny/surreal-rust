//! Window management for the editor.

use std::time::Duration;

use egui::{CursorIcon, RawInput};
use surreal::{
  collections::FastHashMap,
  graphics::{GraphicsServer, Image, ImageFormat},
  input::{InputServer, Key},
  maths::vec2,
  ui::UserInterface,
};
use winit::{
  event::{Event, ModifiersState, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Icon, Window, WindowId},
};

/// Top-level host for [`EditorWindow`]s in the application.
///
/// The window manager is responsible for managing the lifetime of all windows
/// in the application, as well as dispatching events to the appropriate window.
pub struct EditorWindowHost {
  event_loop: EventLoop<()>,
  windows: FastHashMap<WindowId, WindowState>,
}

/// Internal state for a [`EditorWindow`] and it's associated `winit`
/// [`Window`].
struct WindowState {
  winit_window: Window,
  editor_window: Box<dyn EditorWindow>,
  graphics_server: GraphicsServer,
  input_server: InputServer,
  user_interface: UserInterface,
  user_interface_state: UserInterfaceState,
}

/// Internal state for a [`WindowState`]'s [`UserInterface`].
struct UserInterfaceState {
  pixels_per_point: f32,
  raw_input: RawInput,
  egui_cursor_icon: CursorIcon,
  winit_cursor_icon: winit::window::CursorIcon,
  exclusive_keyboard_input: bool,
  exclusive_pointer_input: bool,
  is_focused: bool,
}

/// A window that can be shown in the [`EditorWindowHost`].
pub trait EditorWindow {
  /// Creates the associated [`Window`] for this editor window.
  fn create_window(&self) -> winit::window::WindowBuilder {
    winit::window::WindowBuilder::new()
      .with_title("Surreal")
      .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
      .with_resizable(true)
      .with_visible(true)
      .with_window_icon(Some(load_editor_icon()))
  }

  /// Called when the window should update.
  fn on_update(&mut self) {}

  /// Called when the window should redraw.
  fn on_draw(&mut self, _graphics: &GraphicsServer) {}

  /// Called when the window should render it's UI.
  fn on_ui(&mut self, _ctx: &egui::Context) {}

  /// Called when the window receives a [`WindowEvent`].
  fn on_event(&mut self, _event: &WindowEvent) {}
}

impl EditorWindowHost {
  /// Creates a new [`EditorWindowHost`].
  pub fn new() -> Self {
    Self {
      event_loop: EventLoop::new(),
      windows: FastHashMap::default(),
    }
  }

  /// Adds a new [`EditorWindow`] to the manager.
  pub fn add_window(&mut self, editor_window: impl EditorWindow + 'static) {
    // set-up the OpenGL context for the window, too
    let window = editor_window
      .create_window()
      .build(&self.event_loop)
      .expect("Failed to build window window");

    // set-up graphics server and user interface
    let pixels_per_point = window.scale_factor() as f32;
    let graphics_server = GraphicsServer::opengl(&window, true, 1).expect("Failed to build graphics server");
    let input_server = InputServer::new(pixels_per_point);
    let user_interface = UserInterface::new(&graphics_server);

    self.windows.insert(
      window.id(),
      WindowState {
        winit_window: window,
        editor_window: Box::new(editor_window),
        user_interface,
        graphics_server,
        input_server,
        user_interface_state: UserInterfaceState {
          pixels_per_point,
          raw_input: RawInput::default(),
          egui_cursor_icon: CursorIcon::None,
          winit_cursor_icon: winit::window::CursorIcon::Default,
          exclusive_keyboard_input: false,
          exclusive_pointer_input: false,
          is_focused: false,
        },
      },
    );
  }

  /// Runs the event loop for the manager.
  ///
  /// This method will block the current thread until the application is closed.
  pub fn run(mut self) {
    surreal::diagnostics::trace!("Entering editor window loop");

    self.event_loop.run(move |event, _, control_flow| match event {
      Event::MainEventsCleared => {
        for (window_id, state) in &mut self.windows {
          surreal::diagnostics::profile_scope!("Update window", &format!("{:?}", window_id));

          // TODO: clean this up
          state.user_interface_state.raw_input = state.input_server.raw_input.clone();

          state.editor_window.on_update();
          state.winit_window.request_redraw();

          if state.user_interface_state.is_focused {
            *control_flow = ControlFlow::Poll;
          } else {
            *control_flow = ControlFlow::Wait;
          }
        }

        surreal::diagnostics::finish_frame();
      }
      Event::RedrawRequested(window_id) => {
        if let Some(state) = self.windows.get_mut(&window_id) {
          surreal::diagnostics::profile_scope!("Redraw window", &format!("{:?}", window_id));

          state.graphics_server.begin_frame();

          if let Some(keyboard) = &state.input_server.keyboard {
            if keyboard.is_key_pressed(Key::F7) {
              state.user_interface.toggle_profiler();
            }
          }

          state.editor_window.on_draw(&state.graphics_server);
          state.user_interface.run(&mut state.user_interface_state, |context| {
            state.editor_window.on_ui(context);
          });

          state.input_server.tick();

          state.graphics_server.end_frame();
        }
      }
      Event::WindowEvent { window_id, event } => {
        if let Some(state) = self.windows.get_mut(&window_id) {
          // forward down to the window, before handling
          state.editor_window.on_event(&event);

          match event {
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
              state.input_server.pixels_per_point = scale_factor as f32;

              surreal::diagnostics::trace!("Window scale factor changed to {}", scale_factor);
            }
            WindowEvent::CursorMoved { position, .. } => {
              let size = state.winit_window.inner_size();

              state.input_server.on_mouse_move(
                vec2(position.x as f32, position.y as f32),
                vec2(size.width as f32, size.height as f32),
              );
            }
            WindowEvent::MouseWheel { delta, .. } => {
              state.input_server.on_mouse_wheel(&delta);
            }
            WindowEvent::MouseInput {
              button,
              state: element_state,
              ..
            } => {
              state.input_server.on_mouse_button(button, element_state);
            }
            WindowEvent::KeyboardInput { input, .. } => {
              state.input_server.on_keyboard_event(&input);
            }
            WindowEvent::ReceivedCharacter(character) => {
              state.input_server.on_character_received(character);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
              state.input_server.on_modifiers_changed(modifiers);
            }
            WindowEvent::Focused(focused) => {
              state.user_interface_state.is_focused = focused;
              state.input_server.on_modifiers_changed(ModifiersState::default());

              if focused {
                surreal::diagnostics::trace!("Window gained focus");
              } else {
                surreal::diagnostics::trace!("Window lost focus");
              }
            }
            WindowEvent::Resized(size) => {
              state.graphics_server.set_viewport_size(size);

              surreal::diagnostics::trace!("Window resized to {}x{}", size.width, size.height);
            }
            WindowEvent::CloseRequested => {
              self.windows.remove(&window_id);

              // no more windows left? we're done
              if self.windows.len() == 0 {
                *control_flow = ControlFlow::Exit;
              }
            }
            _ => {}
          }
        }
      }
      _ => {}
    });
  }
}

impl surreal::ui::UserInterfaceHost for UserInterfaceState {
  fn pixels_per_point(&self) -> f32 {
    self.pixels_per_point as f32
  }

  fn raw_input(&self) -> &RawInput {
    &self.raw_input
  }

  fn set_exclusive_keyboard_input(&mut self, exclusive: bool) {
    self.exclusive_keyboard_input = exclusive;
  }

  fn set_exclusive_pointer_input(&mut self, exclusive: bool) {
    self.exclusive_pointer_input = exclusive;
  }

  fn set_cursor_icon(&mut self, cursor_icon: CursorIcon) {
    fn convert_cursor(cursor_icon: CursorIcon) -> Option<winit::window::CursorIcon> {
      match cursor_icon {
        CursorIcon::None => None,

        CursorIcon::Alias => Some(winit::window::CursorIcon::Alias),
        CursorIcon::AllScroll => Some(winit::window::CursorIcon::AllScroll),
        CursorIcon::Cell => Some(winit::window::CursorIcon::Cell),
        CursorIcon::ContextMenu => Some(winit::window::CursorIcon::ContextMenu),
        CursorIcon::Copy => Some(winit::window::CursorIcon::Copy),
        CursorIcon::Crosshair => Some(winit::window::CursorIcon::Crosshair),
        CursorIcon::Default => Some(winit::window::CursorIcon::Default),
        CursorIcon::Grab => Some(winit::window::CursorIcon::Grab),
        CursorIcon::Grabbing => Some(winit::window::CursorIcon::Grabbing),
        CursorIcon::Help => Some(winit::window::CursorIcon::Help),
        CursorIcon::Move => Some(winit::window::CursorIcon::Move),
        CursorIcon::NoDrop => Some(winit::window::CursorIcon::NoDrop),
        CursorIcon::NotAllowed => Some(winit::window::CursorIcon::NotAllowed),
        CursorIcon::PointingHand => Some(winit::window::CursorIcon::Hand),
        CursorIcon::Progress => Some(winit::window::CursorIcon::Progress),

        CursorIcon::ResizeHorizontal => Some(winit::window::CursorIcon::EwResize),
        CursorIcon::ResizeNeSw => Some(winit::window::CursorIcon::NeswResize),
        CursorIcon::ResizeNwSe => Some(winit::window::CursorIcon::NwseResize),
        CursorIcon::ResizeVertical => Some(winit::window::CursorIcon::NsResize),

        CursorIcon::ResizeEast => Some(winit::window::CursorIcon::EResize),
        CursorIcon::ResizeSouthEast => Some(winit::window::CursorIcon::SeResize),
        CursorIcon::ResizeSouth => Some(winit::window::CursorIcon::SResize),
        CursorIcon::ResizeSouthWest => Some(winit::window::CursorIcon::SwResize),
        CursorIcon::ResizeWest => Some(winit::window::CursorIcon::WResize),
        CursorIcon::ResizeNorthWest => Some(winit::window::CursorIcon::NwResize),
        CursorIcon::ResizeNorth => Some(winit::window::CursorIcon::NResize),
        CursorIcon::ResizeNorthEast => Some(winit::window::CursorIcon::NeResize),
        CursorIcon::ResizeColumn => Some(winit::window::CursorIcon::ColResize),
        CursorIcon::ResizeRow => Some(winit::window::CursorIcon::RowResize),

        CursorIcon::Text => Some(winit::window::CursorIcon::Text),
        CursorIcon::VerticalText => Some(winit::window::CursorIcon::VerticalText),
        CursorIcon::Wait => Some(winit::window::CursorIcon::Wait),
        CursorIcon::ZoomIn => Some(winit::window::CursorIcon::ZoomIn),
        CursorIcon::ZoomOut => Some(winit::window::CursorIcon::ZoomOut),
      }
    }

    // prevent flickering near frame boundary when Windows OS tries to control
    // cursor icon for window resizing
    if self.egui_cursor_icon == cursor_icon {
      return;
    }

    self.egui_cursor_icon = cursor_icon;

    if let Some(cursor_icon) = convert_cursor(cursor_icon) {
      self.winit_cursor_icon = cursor_icon;
    } else {
      self.winit_cursor_icon = winit::window::CursorIcon::Default;
    }
  }

  fn request_redraw(&mut self) {
    // TODO: implement me
  }

  fn request_redraw_after(&mut self, _duration: Duration) {
    // TODO: implement me
  }
}

/// Loads the default [`Icon`] for the editor.
pub fn load_editor_icon() -> Icon {
  const ICON_DATA: &[u8] = include_bytes!("../../../surreal.ico");

  Image::from_buffer(ICON_DATA, ImageFormat::Ico)
    .and_then(|image| image.to_icon())
    .expect("Failed to convert icon from raw image")
}
