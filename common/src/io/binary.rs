/// Allows a type to be converted to a byte slice.
pub trait ToBinary {
  /// Converts the type to a byte slice.
  fn to_binary(&self) -> Vec<u8>;
}

/// Allows a type to be constructed from a byte slice.
pub trait FromBinary {
  /// Constructs the type from a byte slice.
  fn from_binary(bytes: &[u8]) -> Self;
}

macro_rules! impl_binary {
  ($type:ty) => {
    impl ToBinary for $type {
      #[inline]
      fn to_binary(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
      }
    }

    impl FromBinary for $type {
      #[inline]
      fn from_binary(bytes: &[u8]) -> Self {
        let mut array = [0; std::mem::size_of::<$type>()];
        array.copy_from_slice(bytes);
        Self::from_le_bytes(array)
      }
    }
  };
}

impl_binary!(u8);
impl_binary!(u16);
impl_binary!(u32);
impl_binary!(u64);
impl_binary!(u128);
impl_binary!(usize);
impl_binary!(i8);
impl_binary!(i16);
impl_binary!(i32);
impl_binary!(i64);
impl_binary!(i128);
impl_binary!(isize);
impl_binary!(f32);
impl_binary!(f64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_binary_for_simple_types() {
    assert_eq!(42u8.to_binary(), vec![42]);
    assert_eq!(42u16.to_binary(), vec![42, 0]);
    assert_eq!(42u32.to_binary(), vec![42, 0, 0, 0]);
    assert_eq!(42u64.to_binary(), vec![42, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(42i8.to_binary(), vec![42]);
    assert_eq!(42i16.to_binary(), vec![42, 0]);
    assert_eq!(42i32.to_binary(), vec![42, 0, 0, 0]);
    assert_eq!(42i64.to_binary(), vec![42, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(42.0f32.to_binary(), vec![0, 0, 0x28, 0x42]);
    assert_eq!(42.0f64.to_binary(), vec![0, 0, 0, 0, 0, 0, 69, 64]);
  }

  #[test]
  fn test_from_binary_for_simple_types() {
    assert_eq!(u8::from_binary(&[42]), 42);
    assert_eq!(u16::from_binary(&[42, 0]), 42);
    assert_eq!(u32::from_binary(&[42, 0, 0, 0]), 42);
    assert_eq!(u64::from_binary(&[42, 0, 0, 0, 0, 0, 0, 0]), 42);
    assert_eq!(i8::from_binary(&[42]), 42);
    assert_eq!(i16::from_binary(&[42, 0]), 42);
    assert_eq!(i32::from_binary(&[42, 0, 0, 0]), 42);
    assert_eq!(i64::from_binary(&[42, 0, 0, 0, 0, 0, 0, 0]), 42);
    assert_eq!(f32::from_binary(&[0, 0, 0x28, 0x42]), 42.0);
    assert_eq!(f64::from_binary(&[0, 0, 0, 0, 0, 0, 69, 64]), 42.0);
  }
}
