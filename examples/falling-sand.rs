use common::{DeltaClock, FromRandom, Lerp, PingPong};
use graphics::Color32;
use surreal::backends::sdl::*;

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Falling Sand",
    ..Default::default()
  })
  .expect("Failed to create window");

  let renderer = Renderer::new(&window);
  let texture = Texture::new(&renderer, 256, 144);

  let mut bitmap = Bitmap::new(256, 144);

  let color1 = Color32::random();
  let color2 = Color32::random();

  let mut total_time = 0.;
  let mut delta_clock = DeltaClock::default();

  while window.update() {
    total_time += delta_clock.tick();

    bitmap.fill(Color32::lerp(color1, color2, total_time.ping_pong()));

    bitmap.blit_to_texture(&texture);
    texture.blit_to_display(&renderer);

    window.present();
  }
}

/// A simple bitmap of pixels.
#[derive(Clone)]
struct Bitmap {
  pixels: Vec<Color32>,
}

impl Bitmap {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      pixels: vec![Color32::BLACK; width * height],
    }
  }

  pub fn fill(&mut self, color: Color32) {
    self.pixels.fill(color);
  }

  pub fn blit_to_texture(&self, texture: &Texture) {
    texture.copy_pixels(&self.pixels);
  }
}

/// A wrapper over an SDL renderer.
struct Renderer(*mut sys::SDL_Renderer);

impl Renderer {
  pub fn new(window: &Window) -> Self {
    let renderer = unsafe {
      sys::SDL_CreateRenderer(
        window.get_sdl_window(),
        -1,
        sys::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32,
      )
    };

    Self(renderer)
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    unsafe {
      sys::SDL_DestroyRenderer(self.0);
    }
  }
}

/// A wrapper over an SDL texture.
struct Texture(*mut sys::SDL_Texture);

impl Texture {
  pub fn new(renderer: &Renderer, width: usize, height: usize) -> Self {
    let texture = unsafe {
      sys::SDL_CreateTexture(
        renderer.0,
        sys::SDL_PixelFormatEnum::SDL_PIXELFORMAT_RGBA32 as u32,
        sys::SDL_TextureAccess::SDL_TEXTUREACCESS_STREAMING as i32,
        width as i32,
        height as i32,
      )
    };

    Self(texture)
  }

  pub fn copy_pixels(&self, pixels: &[Color32]) {
    unsafe {
      sys::SDL_UpdateTexture(
        self.0,
        std::ptr::null(),
        pixels.as_ptr() as *const std::ffi::c_void,
        (256 * std::mem::size_of::<Color32>()) as i32,
      );
    }
  }

  pub fn blit_to_display(&self, renderer: &Renderer) {
    unsafe {
      sys::SDL_RenderCopy(renderer.0, self.0, std::ptr::null(), std::ptr::null());
      sys::SDL_RenderPresent(renderer.0);
    }
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      sys::SDL_DestroyTexture(self.0);
    }
  }
}
