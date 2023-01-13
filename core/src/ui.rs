//! User interface rendering and widgets.

use std::time::Duration;

pub use egui;

use crate::{
  self as surreal,
  collections::FastHashMap,
  diagnostics,
  graphics::*,
  maths::{vec2, Rectangle},
};

/// A shader program to use for egui canvas rendering.
const SHADER_CANVAS_STANDARD: &str = include_str!("../../assets/shaders/canvas-standard.glsl");

/// A host for the [`UserInterface`] and provider for [`egui::RawInput`] .
///
/// This provider adapts some source host to allow platform control and queries.
pub trait UserInterfaceHost {
  // ui input
  fn pixels_per_point(&self) -> f32;
  fn raw_input(&self) -> &egui::RawInput;

  // control and platform output
  fn set_exclusive_keyboard_input(&mut self, exclusive: bool);
  fn set_exclusive_pointer_input(&mut self, exclusive: bool);
  fn set_cursor_icon(&mut self, cursor_icon: egui::CursorIcon);
  fn request_redraw(&mut self);
  fn request_redraw_after(&mut self, duration: Duration);
}

// TODO: implement me, remove old host interface

/// A hosting container for [`UserInterface`] construction.
pub struct UserInterfaceContainer {
  _user_interface: UserInterface,
}

impl UserInterfaceContainer {
  /// Creates a new [`UserInterfaceContainer`].
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      _user_interface: UserInterface::new(graphics),
    }
  }

  /// Receives and processes a [`winit::event::WindowEvent`].
  pub fn receive_event(&mut self, _window_event: &winit::event::WindowEvent) {
    todo!()
  }
}

/// A canvas for immediate mode user interface rendering via `egui`.
pub struct UserInterface {
  graphics: GraphicsServer,
  context: egui::Context,
  material: Material,
  mesh: Mesh<Vertex2>,
  textures: FastHashMap<egui::TextureId, Texture>,
  is_profiler_open: bool,
}

impl UserInterface {
  /// Creates a new user interface.
  pub fn new(graphics: &GraphicsServer) -> Self {
    // load and configure material
    let shader = ShaderProgram::from_glsl(graphics, SHADER_CANVAS_STANDARD).unwrap();
    let mut material = Material::new(graphics, &shader);

    material.set_culling_mode(CullingMode::Disabled);
    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::One,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    Self {
      graphics: graphics.clone(),
      context: egui::Context::default(),
      material,
      mesh: Mesh::new(graphics, BufferUsage::Dynamic),
      textures: FastHashMap::default(),
      is_profiler_open: false,
    }
  }

  /// Sets the style of the user interface to a light mode.
  pub fn set_light_mode(&mut self) {
    self.context.set_style(egui::Style {
      visuals: egui::Visuals::light(),
      ..Default::default()
    });
  }

  /// Sets the style of the user interface to a dark mode (default).
  pub fn set_dark_mode(&mut self) {
    self.context.set_style(egui::Style {
      visuals: egui::Visuals::dark(),
      ..Default::default()
    });
  }

  /// Toggles the profiler window open/closed.
  pub fn toggle_profiler(&mut self) {
    self.is_profiler_open = !self.is_profiler_open;

    if self.is_profiler_open {
      diagnostics::enable_profiling()
    } else {
      diagnostics::disable_profiling()
    }
  }

  /// Begins a new frame.
  #[diagnostics::profiling]
  pub fn begin_frame(&mut self, host: &mut dyn UserInterfaceHost) {
    let raw_input = host.raw_input();
    let pixels_per_point = host.pixels_per_point();

    self.context.set_pixels_per_point(pixels_per_point);
    self.context.begin_frame(raw_input.clone());
  }

  /// Ends the current frame.
  #[diagnostics::profiling]
  pub fn end_frame(&mut self, host: &mut dyn UserInterfaceHost) {
    let full_output = self.context.end_frame();
    let textures_delta = full_output.textures_delta;

    for (id, image_delta) in textures_delta.set {
      // convert image representations to our color format and collect width and
      // height
      let (pixels, [width, height]) = match image_delta.image {
        egui::ImageData::Color(image) => {
          let pixels = image
            .pixels
            .iter()
            .map(|pixel| Color32::rgba(pixel.r(), pixel.g(), pixel.b(), pixel.a()))
            .collect::<Vec<_>>();

          (pixels, image.size)
        }
        egui::ImageData::Font(image) => {
          let pixels = image
            .pixels
            .iter()
            .map(|pixel| {
              Color32::rgba(
                (*pixel * 255.0) as u8,
                (*pixel * 255.0) as u8,
                (*pixel * 255.0) as u8,
                (*pixel * 255.0) as u8,
              )
            })
            .collect::<Vec<_>>();

          (pixels, image.size)
        }
      };

      match image_delta.pos {
        None => {
          // create new texture
          let texture = Texture::new(&self.graphics);

          texture.write_pixels(width, height, &pixels);

          self.textures.insert(id, texture);
        }
        Some([x, y]) => {
          // update existing texture
          let texture = self.textures.get_mut(&id).expect("Texture not found");
          let region = Rectangle::from_corner_points(x as f32, y as f32, x as f32 + width as f32, y as f32 + height as f32);

          texture.write_sub_pixels(&region, &pixels);
        }
      }
    }

    // free textures that are no longer in use
    for id in textures_delta.free {
      self.textures.remove(&id);
    }

    // compute display size
    let pixels_per_point = self.context.pixels_per_point();
    let (width_in_pixels, height_in_pixels) = self.graphics.viewport_size();
    let (width_in_points, height_in_points) = (
      width_in_pixels as f32 / pixels_per_point,
      height_in_pixels as f32 / pixels_per_point,
    );

    // create meshes from shapes
    for clipped_primitive in self.context.tessellate(full_output.shapes) {
      match clipped_primitive.primitive {
        egui::epaint::Primitive::Mesh(mesh) => {
          let vertices = mesh.vertices;
          let indices = mesh.indices;
          let texture = self.textures.get(&mesh.texture_id).unwrap();

          // update our single mesh shape and re-render it
          self.mesh.with_buffers(|vertex_buffer, index_buffer| {
            // our vertices are blit-ably the same as what egui gives us, so just cast the
            // slice.
            let vertices = unsafe { std::slice::from_raw_parts(vertices.as_ptr() as *const Vertex2, vertices.len()) };

            vertex_buffer.write_data(vertices);
            index_buffer.write_data(&indices);
          });

          // compute scissor rect based on clip position
          let clip_rect = clipped_primitive.clip_rect;

          let clip_min_x = pixels_per_point * clip_rect.min.x;
          let clip_min_y = pixels_per_point * clip_rect.min.y;
          let clip_max_x = pixels_per_point * clip_rect.max.x;
          let clip_max_y = pixels_per_point * clip_rect.max.y;

          let clip_min_x = clip_min_x.clamp(0.0, width_in_pixels as f32);
          let clip_min_y = clip_min_y.clamp(0.0, height_in_pixels as f32);
          let clip_max_x = clip_max_x.clamp(clip_min_x, width_in_pixels as f32);
          let clip_max_y = clip_max_y.clamp(clip_min_y, height_in_pixels as f32);

          let clip_min_x = clip_min_x.round() as i32;
          let clip_min_y = clip_min_y.round() as i32;
          let clip_max_x = clip_max_x.round() as i32;
          let clip_max_y = clip_max_y.round() as i32;

          // configure material properties
          self.material.set_scissor_mode(ScissorMode::Enabled {
            left: clip_min_x,
            bottom: height_in_pixels as i32 - clip_max_y,
            width: clip_max_x - clip_min_x,
            height: clip_max_y - clip_min_y,
          });

          let screen_size = vec2(width_in_points, height_in_points);

          self.material.set_uniform("u_viewportSize", screen_size);
          self.material.set_texture("u_texture", texture, None);

          // render mesh using material
          self.mesh.draw(&self.material, PrimitiveTopology::Triangles);
        }
        egui::epaint::Primitive::Callback(_) => {
          panic!("Custom rendering callbacks not yet supported")
        }
      }
    }

    let platform_output = full_output.platform_output;
    let repaint_after = full_output.repaint_after;

    host.set_exclusive_keyboard_input(self.context.wants_keyboard_input());
    host.set_exclusive_pointer_input(self.context.wants_pointer_input());
    host.set_cursor_icon(platform_output.cursor_icon);

    // TODO: handle clipboard, too

    if repaint_after.is_zero() {
      host.request_redraw();
    } else if repaint_after.as_secs() > 0 && repaint_after.as_secs() < Duration::MAX.as_secs() {
      host.request_redraw_after(full_output.repaint_after);
    }
  }

  /// Propagates input into the user interface and runs the given body against
  /// the UI.
  ///
  /// Note: this is a fairly expensive operation and should ideally be done once
  /// per frame,       with as much UI work as possible within a single call.
  #[diagnostics::profiling]
  pub fn run(&mut self, host: &mut dyn UserInterfaceHost, mut body: impl FnMut(&egui::Context)) {
    self.begin_frame(host);

    body(&self.context);

    if self.is_profiler_open {
      self.is_profiler_open = puffin_egui::profiler_window(&self.context);
    }

    self.end_frame(host);
  }
}
