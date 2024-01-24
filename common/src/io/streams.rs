use std::io::{BufRead, Seek, Write};

/// A stream for reading from some [`VirtualPath`].
pub trait InputStream: Seek + BufRead {
  fn read_u8(&mut self) -> crate::Result<u8>;
  fn read_u16(&mut self) -> crate::Result<u16>;
  fn read_u32(&mut self) -> crate::Result<u32>;
  fn read_u64(&mut self) -> crate::Result<u64>;
  fn read_u128(&mut self) -> crate::Result<u128>;
  fn read_usize(&mut self) -> crate::Result<usize>;
  fn read_i8(&mut self) -> crate::Result<i8>;
  fn read_i16(&mut self) -> crate::Result<i16>;
  fn read_i32(&mut self) -> crate::Result<i32>;
  fn read_i64(&mut self) -> crate::Result<i64>;
  fn read_i128(&mut self) -> crate::Result<i128>;
  fn read_isize(&mut self) -> crate::Result<isize>;
  fn read_f32(&mut self) -> crate::Result<f32>;
  fn read_f64(&mut self) -> crate::Result<f64>;
  fn read_string(&mut self) -> crate::Result<String>;
  fn read_bytes(&mut self, amount: usize) -> crate::Result<Vec<u8>>;
  fn to_buffer(&mut self) -> crate::Result<Vec<u8>>;
  fn to_string(&mut self) -> crate::Result<String>;
}

macro_rules! impl_read {
  ($self:expr, $buffer_size:expr, $result:ty) => {{
    let mut buffer = [0; $buffer_size];

    $self.read_exact(&mut buffer)?;

    Ok(<$result>::from_le_bytes(buffer))
  }};
}

/// Blanket implementation of [`InputStream`].
impl<T: BufRead + Seek> InputStream for T {
  #[inline]
  fn read_u8(&mut self) -> crate::Result<u8> {
    impl_read!(self, 1, u8)
  }

  #[inline]
  fn read_u16(&mut self) -> crate::Result<u16> {
    impl_read!(self, 2, u16)
  }

  #[inline]
  fn read_u32(&mut self) -> crate::Result<u32> {
    impl_read!(self, 4, u32)
  }

  #[inline]
  fn read_u64(&mut self) -> crate::Result<u64> {
    impl_read!(self, 8, u64)
  }

  #[inline]
  fn read_u128(&mut self) -> crate::Result<u128> {
    impl_read!(self, 16, u128)
  }

  #[inline]
  fn read_usize(&mut self) -> crate::Result<usize> {
    impl_read!(self, std::mem::size_of::<usize>(), usize)
  }

  #[inline]
  fn read_i8(&mut self) -> crate::Result<i8> {
    impl_read!(self, 1, i8)
  }

  #[inline]
  fn read_i16(&mut self) -> crate::Result<i16> {
    impl_read!(self, 2, i16)
  }

  #[inline]
  fn read_i32(&mut self) -> crate::Result<i32> {
    impl_read!(self, 4, i32)
  }

  #[inline]
  fn read_i64(&mut self) -> crate::Result<i64> {
    impl_read!(self, 8, i64)
  }

  #[inline]
  fn read_i128(&mut self) -> crate::Result<i128> {
    impl_read!(self, 16, i128)
  }

  #[inline]
  fn read_isize(&mut self) -> crate::Result<isize> {
    impl_read!(self, std::mem::size_of::<isize>(), isize)
  }

  #[inline]
  fn read_f32(&mut self) -> crate::Result<f32> {
    impl_read!(self, 4, f32)
  }

  #[inline]
  fn read_f64(&mut self) -> crate::Result<f64> {
    impl_read!(self, 8, f64)
  }

  fn read_string(&mut self) -> crate::Result<String> {
    let length = self.read_usize()?;
    let mut buffer = vec![0; length];

    self.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
  }

  fn read_bytes(&mut self, amount: usize) -> crate::Result<Vec<u8>> {
    let mut buffer = vec![0; amount];

    self.read_exact(&mut buffer)?;

    Ok(buffer)
  }

  fn to_buffer(&mut self) -> crate::Result<Vec<u8>> {
    let mut buffer = Vec::new();

    self.read_to_end(&mut buffer)?;

    Ok(buffer)
  }

  fn to_string(&mut self) -> crate::Result<String> {
    let mut buffer = String::new();

    self.read_to_string(&mut buffer)?;

    Ok(buffer)
  }
}

/// A stream for writing to some [`VirtualPath`].
pub trait OutputStream: Seek + Write {
  fn write_u8(&mut self, value: u8) -> crate::Result<()>;
  fn write_u16(&mut self, value: u16) -> crate::Result<()>;
  fn write_u32(&mut self, value: u32) -> crate::Result<()>;
  fn write_u64(&mut self, value: u64) -> crate::Result<()>;
  fn write_u128(&mut self, value: u128) -> crate::Result<()>;
  fn write_usize(&mut self, value: usize) -> crate::Result<()>;
  fn write_i8(&mut self, value: i8) -> crate::Result<()>;
  fn write_i16(&mut self, value: i16) -> crate::Result<()>;
  fn write_i32(&mut self, value: i32) -> crate::Result<()>;
  fn write_i64(&mut self, value: i64) -> crate::Result<()>;
  fn write_i128(&mut self, value: i128) -> crate::Result<()>;
  fn write_isize(&mut self, value: isize) -> crate::Result<()>;
  fn write_f32(&mut self, value: f32) -> crate::Result<()>;
  fn write_f64(&mut self, value: f64) -> crate::Result<()>;
  fn write_string(&mut self, value: &str) -> crate::Result<()>;
  fn write_bytes(&mut self, value: &[u8]) -> crate::Result<()>;
}

macro_rules! impl_write {
  ($self:expr, $type:ty, $value:expr) => {{
    let buffer = <$type>::to_le_bytes($value);

    $self.write_all(&buffer)?;

    Ok(())
  }};
}

/// Blanket implementation of [`OutputStream`].
impl<T: Write + Seek> OutputStream for T {
  #[inline]
  fn write_u8(&mut self, value: u8) -> crate::Result<()> {
    impl_write!(self, u8, value)
  }

  #[inline]
  fn write_u16(&mut self, value: u16) -> crate::Result<()> {
    impl_write!(self, u16, value)
  }

  #[inline]
  fn write_u32(&mut self, value: u32) -> crate::Result<()> {
    impl_write!(self, u32, value)
  }

  #[inline]
  fn write_u64(&mut self, value: u64) -> crate::Result<()> {
    impl_write!(self, u64, value)
  }

  #[inline]
  fn write_u128(&mut self, value: u128) -> crate::Result<()> {
    impl_write!(self, u128, value)
  }

  #[inline]
  fn write_usize(&mut self, value: usize) -> crate::Result<()> {
    impl_write!(self, usize, value)
  }

  #[inline]
  fn write_i8(&mut self, value: i8) -> crate::Result<()> {
    impl_write!(self, i8, value)
  }

  #[inline]
  fn write_i16(&mut self, value: i16) -> crate::Result<()> {
    impl_write!(self, i16, value)
  }

  #[inline]
  fn write_i32(&mut self, value: i32) -> crate::Result<()> {
    impl_write!(self, i32, value)
  }

  #[inline]
  fn write_i64(&mut self, value: i64) -> crate::Result<()> {
    impl_write!(self, i64, value)
  }

  #[inline]
  fn write_i128(&mut self, value: i128) -> crate::Result<()> {
    impl_write!(self, i128, value)
  }

  #[inline]
  fn write_isize(&mut self, value: isize) -> crate::Result<()> {
    impl_write!(self, isize, value)
  }

  #[inline]
  fn write_f32(&mut self, value: f32) -> crate::Result<()> {
    impl_write!(self, f32, value)
  }

  #[inline]
  fn write_f64(&mut self, value: f64) -> crate::Result<()> {
    impl_write!(self, f64, value)
  }

  fn write_string(&mut self, value: &str) -> crate::Result<()> {
    self.write_usize(value.len())?;
    self.write_bytes(value.as_bytes())?;

    Ok(())
  }

  fn write_bytes(&mut self, value: &[u8]) -> crate::Result<()> {
    self.write_all(value)?;

    Ok(())
  }
}
