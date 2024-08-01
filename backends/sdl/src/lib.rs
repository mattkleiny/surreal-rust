//! SDL bindings for Surreal.

use std::ffi::{c_int, CString};

use sdl2_sys::{
  SDL_GLattr::{
    SDL_GL_CONTEXT_FLAGS, SDL_GL_CONTEXT_MAJOR_VERSION, SDL_GL_CONTEXT_MINOR_VERSION, SDL_GL_CONTEXT_PROFILE_MASK,
  },
  SDL_GLcontextFlag::SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG,
  SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_CORE,
};

mod audio;
mod graphics;
mod input;

/// Represents an error that can occur when creating a window.
#[derive(Debug)]
pub enum WindowError {
  FailedToInitialize,
  FailedToCreateWindow,
  FailedToCreateRenderer,
}

/// Represents a window.
pub struct Window {
  window: *mut sdl2_sys::SDL_Window,
  gl_context: sdl2_sys::SDL_GLContext,
  keyboard_device: input::SdlKeyboardDevice,
  mouse_device: input::SdlMouseDevice,
}

/// Settings for a window.
pub struct WindowSettings {
  pub title: String,
  pub width: u32,
  pub height: u32,
  pub vsync_enabled: bool,
  pub icon: Option<graphics::Image>,
}

impl Default for WindowSettings {
  fn default() -> Self {
    Self {
      title: "Surreal".to_string(),
      width: 1024,
      height: 768,
      vsync_enabled: true,
      icon: None,
    }
  }
}

impl Window {
  /// Creates a new window.
  pub fn new(settings: WindowSettings) -> Result<Self, WindowError> {
    use sdl2_sys::*;

    unsafe {
      // initialize SDL2
      if SDL_Init(SDL_INIT_VIDEO) < 0 {
        return Err(WindowError::FailedToInitialize);
      }

      // build the window
      let mut window_flags = SDL_WindowFlags::SDL_WINDOW_SHOWN as u32;

      window_flags |= SDL_WindowFlags::SDL_WINDOW_OPENGL as u32;
      window_flags |= SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32;

      let title = CString::new(settings.title).unwrap();
      let window = SDL_CreateWindow(
        title.as_ptr() as *const _,
        SDL_WINDOWPOS_CENTERED_MASK as i32,
        SDL_WINDOWPOS_CENTERED_MASK as i32,
        settings.width as i32,
        settings.height as i32,
        window_flags,
      );
      if window.is_null() {
        return Err(WindowError::FailedToCreateWindow);
      }

      SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
      SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 1);
      SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG as c_int);
      SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE as c_int);

      // create the OpenGL context
      let gl_context = SDL_GL_CreateContext(window);
      if gl_context.is_null() {
        return Err(WindowError::FailedToCreateRenderer);
      }

      if settings.vsync_enabled {
        // try adaptive vsync first
        if SDL_GL_SetSwapInterval(-1) == -1 {
          // if that fails, try normal vsync
          SDL_GL_SetSwapInterval(1);
        }
      }

      SDL_GL_MakeCurrent(window, gl_context);
      SDL_GL_LoadLibrary(std::ptr::null());

      let window = Self {
        window,
        gl_context,
        keyboard_device: input::SdlKeyboardDevice::default(),
        mouse_device: input::SdlMouseDevice::default(),
      };

      // set the window icon
      if let Some(icon) = &settings.icon {
        window.set_window_icon(icon);
      }

      audio::AudioServer::install(audio::SdlAudioBackend::new());
      graphics::GraphicsServer::install(graphics::SdlGraphicsBackend::new());

      Ok(window)
    }
  }

  /// Sets the window icon.
  pub fn set_window_icon(&self, icon: &graphics::Image) {
    use sdl2_sys::*;

    unsafe {
      let surface = SDL_CreateRGBSurfaceFrom(
        icon.as_ptr() as *mut _,
        icon.width() as i32,
        icon.height() as i32,
        32,
        icon.width() as i32 * 4,
        0x000000ff,
        0x0000ff00,
        0x00ff0000,
        0xff000000,
      );

      SDL_SetWindowIcon(self.window, surface);
      SDL_FreeSurface(surface);
    }
  }

  /// Runs the main window event pump.
  pub fn update(&mut self) -> bool {
    use sdl2_sys::*;

    unsafe {
      let mut running = true;
      let mut event = SDL_Event {
        type_: SDL_EventType::SDL_FIRSTEVENT as u32,
      };

      while SDL_PollEvent(&mut event) != 0 {
        if event.type_ == SDL_EventType::SDL_QUIT as u32 {
          running = false;
        }

        if event.type_ == SDL_EventType::SDL_KEYDOWN as u32 {
          if let Some(virtual_key) = input::virtualkey_from_scancode(event.key.keysym.sym) {
            self.keyboard_device.keyboard_state.insert(virtual_key);
          }
        }

        if event.type_ == SDL_EventType::SDL_KEYUP as u32 {
          if let Some(virtual_key) = input::virtualkey_from_scancode(event.key.keysym.sym) {
            self.keyboard_device.keyboard_state.remove(&virtual_key);
          }
        }
      }

      running
    }
  }

  /// Gets the keyboard device.
  pub fn keyboard(&self) -> &dyn input::KeyboardDevice {
    &self.keyboard_device
  }

  /// Gets the mouse device.
  pub fn mouse(&self) -> &dyn input::MouseDevice {
    &self.mouse_device
  }

  /// Presents the window to the display.
  pub fn present(&self) {
    use sdl2_sys::*;

    unsafe {
      SDL_GL_SwapWindow(self.window);
    }
  }

  /// Gets the raw underlying SDL2 window handle.
  pub fn get_sdl_window(&self) -> *mut sdl2_sys::SDL_Window {
    self.window
  }
}

impl common::Clipboard for Window {
  fn get_clipboard(&self) -> Option<String> {
    unsafe {
      let string = CString::from_raw(sdl2_sys::SDL_GetClipboardText());
      string.into_string().ok()
    }
  }

  fn set_clipboard(&mut self, text: String) {
    unsafe {
      let string = CString::new(text).unwrap();
      sdl2_sys::SDL_SetClipboardText(string.as_ptr());
    }
  }
}

impl Drop for Window {
  /// Destroys the window.
  fn drop(&mut self) {
    use sdl2_sys::*;

    unsafe {
      SDL_GL_DeleteContext(self.gl_context);
      SDL_DestroyWindow(self.window);

      SDL_Quit();
    }
  }
}
