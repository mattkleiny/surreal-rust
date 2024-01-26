//! SDL bindings for Surreal.

use std::ffi::CString;

pub use sdl2_sys as sys; // re-export the SDL2 bindings

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
}

/// Settings for a window.
pub struct WindowSettings {
  pub title: &'static str,
  pub width: u32,
  pub height: u32,
  pub vsync_enabled: bool,
  pub icon: Option<graphics::Color32Image>,
}

impl Default for WindowSettings {
  fn default() -> Self {
    Self {
      title: "Surreal",
      width: 1024,
      height: 768,
      vsync_enabled: true,
      icon: None,
    }
  }
}

impl Window {
  /// Creates a new window.
  pub fn new(settings: &WindowSettings) -> Result<Self, WindowError> {
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

      let window = Self { window, gl_context };

      // set the window icon
      if let Some(icon) = &settings.icon {
        window.set_window_icon(&icon);
      }

      Ok(window)
    }
  }

  /// Sets the window icon.
  pub fn set_window_icon(&self, icon: &graphics::Color32Image) {
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
  pub fn update(&self) -> bool {
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
      }

      running
    }
  }

  /// Presents the window to the display.
  pub fn present(&self) {
    use sdl2_sys::*;

    unsafe {
      SDL_GL_SwapWindow(self.window);
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

impl graphics::OpenGLHost for Window {
  fn get_proc_address(&self, name: &str) -> *const std::ffi::c_void {
    let name = CString::new(name).unwrap();
    unsafe { sdl2_sys::SDL_GL_GetProcAddress(name.as_ptr() as *const _) as *const _ }
  }
}

impl audio::OpenALHost for Window {
  fn get_proc_address(&self, _name: &str) -> *const std::ffi::c_void {
    todo!()
  }
}
