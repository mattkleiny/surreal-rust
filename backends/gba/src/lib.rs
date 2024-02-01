//! A backend for Surreal specifically for the GameBoy Advance.

#![no_std]

extern crate alloc;

pub use hardware::*;

/// The runtime for the GameBoy Advance.
///
/// This is the facade and main entry point to the runtime hardware.
pub struct GameBoyRuntime {}

impl GameBoyRuntime {
  /// Entry point for the GameBoy runtime.
  ///
  /// This function is called by the GameBoy runtime when the game is started.
  /// It is responsible for initializing the allocator and then starting the
  /// game.
  ///
  /// The game should never return from this function, and this function needs
  /// to be called prior to using any dynamically allocated memory.
  pub fn run(mut body: impl FnMut(&mut Self)) {
    hardware::initialize_heap_allocator();

    let mut runtime = Self {};

    loop {
      body(&mut runtime);
      // TODO: don't eat the CPU?
    }
  }
}

mod hardware {
  //! Hardware abstractions for the GameBoy Advance.
  pub use audio::*;
  pub use display::*;
  pub use input::*;

  mod audio;
  mod display;
  mod input;

  /// The allocator used by the GameBoy runtime.
  ///
  /// This is a simple bump allocator that allocates from a fixed-size heap.
  #[global_allocator]
  static HEAP_ALLOCATOR: embedded_alloc::Heap = embedded_alloc::Heap::empty();

  /// Initializes the heap allocator used by the runtime.
  ///
  /// The hardware of the GBA is very limited, so we can't use a dynamic
  /// allocator. Instead, we use a simple bump allocator that allocates from a
  /// fixed-size heap.
  pub(crate) fn initialize_heap_allocator() {
    use core::mem::MaybeUninit;

    // allocate the main heap
    const HEAP_SIZE: usize = 4069;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

    unsafe {
      HEAP_ALLOCATOR.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE);
    }
  }
}
