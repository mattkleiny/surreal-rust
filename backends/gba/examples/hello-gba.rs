#![no_std]
#![no_main]

use surreal_backend_gba::{ivec2, AudioDevice, Color, DisplayDevice, GameBoyRuntime};

fn main() {
  GameBoyRuntime::run(|runtime| {
    runtime.clear(Color { r: 0, g: 0, b: 0 });

    runtime.draw_rect(ivec2(0, 0), 16, 16, Color { r: 255, g: 0, b: 0 });
    runtime.draw_line(ivec2(0, 0), ivec2(240, 160), Color { r: 0, g: 255, b: 0 });

    runtime.draw_circle(120, 80, 16, Color { r: 0, g: 0, b: 255 });

    runtime.play_sound(|time| if time % 1.0 < 0.5 { 0.5 } else { -0.5 });
  });
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
