/// A bump allocator allows allocating of arbitrary data inside of a single fixed-size block.
#[derive(Clone, Debug)]
pub struct BumpAllocator {
  buffer: Vec<u8>,
  position: usize,
}

impl BumpAllocator {
  pub fn new(capacity: usize) -> Self {
    Self {
      buffer: vec![0; capacity],
      position: 0,
    }
  }

  pub fn capacity(&self) -> Size { Size::bytes(self.buffer.len()) }
  pub fn remaining(&self) -> Size { Size::bytes(self.buffer.len() - self.position) }

  pub fn allocate<T>(&mut self) -> T {
    let size = std::mem::size_of::<T>();
    let address = &mut self.buffer[self.position];

    unimplemented!()
  }
}

/// A canonical representation of size, with simple conversions between units.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Size(usize);

impl Size {
  pub const fn bytes(amount: usize) -> Self { Self(amount) }
  pub const fn kilobytes(amount: usize) -> Self { Self::bytes(amount * 1024) }
  pub const fn megabytes(amount: usize) -> Self { Self::kilobytes(amount * 1024) }
  pub const fn gigabytes(amount: usize) -> Self { Self::megabytes(amount * 1024) }

  pub fn as_bytes(&self) -> usize { self.0 }
  pub fn as_kilobytes(&self) -> usize { self.as_bytes() / 1024 }
  pub fn as_megabytes(&self) -> usize { self.as_kilobytes() / 1024 }
  pub fn as_gigabytes(&self) -> usize { self.as_megabytes() / 1024 }
}

#[cfg(test)]
mod tests {
  use crate::maths::Vector2;

  use super::*;

  #[test]
  fn bump_allocator_should_allocate_in_place() {
    let mut allocator = BumpAllocator::new(1024);

    let vector1: Vector2<f32> = allocator.allocate();
    let vector2: Vector2<f32> = allocator.allocate();
    let vector3: Vector2<f32> = allocator.allocate();
  }

  #[test]
  fn size_should_convert_between_scales() {
    let size = Size::gigabytes(1);

    assert_eq!(size.as_gigabytes(), 1);
    assert_eq!(size.as_megabytes(), 1024);
    assert_eq!(size.as_kilobytes(), 1024 * 1024);
    assert_eq!(size.as_bytes(), 1024 * 1024 * 1024);
  }
}