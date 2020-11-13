#![allow(dead_code)]
#![allow(unused_variables)]

#![feature(array_methods)]

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use surreal::graphics::Color;
use surreal::maths::*;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() -> Result<(), Error> {
  let event_loop = EventLoop::new();
  let window = {
    let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

    WindowBuilder::new()
      .with_title("Falling Sand")
      .with_inner_size(size)
      .with_min_inner_size(size)
      .build(&event_loop)
      .unwrap()
  };

  let mut pixels = {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
  };

  let mut simulation = Simulation::new();

  event_loop.run(move |event, _, control_flow| {
    if let Event::RedrawRequested(_) = event {
      simulation.tick();
      simulation.draw(pixels.get_frame());
      pixels.render().expect("Failed to render!");
    }

    if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
      *control_flow = ControlFlow::Exit;
    }

    window.request_redraw();
  });
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pixel {
  Empty,
  Sand,
  Water,
}

impl Random for Pixel {
  fn random(generator: &mut RNG) -> Self {
    let index = generator.next::<u8>() % 3;
    match index {
      1 => Self::Sand,
      2 => Self::Water,
      _ => Self::Empty
    }
  }
}

struct Simulation {
  pixels: DenseGrid<Pixel>
}

impl Simulation {
  pub fn new() -> Self {
    let mut pixels = DenseGrid::new(WIDTH as usize, HEIGHT as usize, Pixel::Empty);
    let mut random = Seed::random().to_random();

    for cell in pixels.cells() {
      pixels.set(cell.x, cell.y, random.next());
    }

    Self { pixels }
  }

  pub fn tick(&mut self) {
    // TODO: implement me
  }

  fn draw(&self, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      let x = (i % WIDTH as usize) as usize;
      let y = (i / WIDTH as usize) as usize;

      let color: Color = match self.pixels.get(x, y) {
        Pixel::Empty => Color::BLACK,
        Pixel::Sand => Color::GREEN,
        Pixel::Water => Color::BLUE,
      };

      pixel.copy_from_slice(&[color.r, color.g, color.b, color.a]);
    }
  }
}
