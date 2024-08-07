use super::*;
use crate::{Color, Color32, Quat, StreamError, StringName, Vec2, Vec3, Vec4};

/// The binary [`Format`].
#[derive(Default)]
pub struct BinaryFormat {}

impl Format for BinaryFormat {
  fn read_chunk(&mut self, stream: &mut dyn InputStream) -> Result<Chunk, StreamError> {
    let chunk_type = stream.read_u8()?;

    match chunk_type {
      0 => {
        let variant = match stream.read_u8()? {
          0 => Variant::Null,
          1 => Variant::Bool(stream.read_u8()? != 0),
          2 => Variant::Char(stream.read_u8()? as char),
          3 => Variant::U8(stream.read_u8()?),
          4 => Variant::U16(stream.read_u16()?),
          5 => Variant::U32(stream.read_u32()?),
          6 => Variant::U64(stream.read_u64()?),
          7 => Variant::I8(stream.read_i8()?),
          8 => Variant::I16(stream.read_i16()?),
          9 => Variant::I32(stream.read_i32()?),
          10 => Variant::I64(stream.read_i64()?),
          11 => Variant::F32(stream.read_f32()?),
          12 => Variant::F64(stream.read_f64()?),
          13 => Variant::String(stream.read_string()?),
          14 => Variant::StringName(StringName::from(stream.read_string()?)),
          15 => Variant::Vec2(Vec2::new(stream.read_f32()?, stream.read_f32()?)),
          16 => Variant::Vec3(Vec3::new(stream.read_f32()?, stream.read_f32()?, stream.read_f32()?)),
          17 => Variant::Vec4(Vec4::new(
            stream.read_f32()?,
            stream.read_f32()?,
            stream.read_f32()?,
            stream.read_f32()?,
          )),
          18 => Variant::Quat(Quat::from_xyzw(
            stream.read_f32()?,
            stream.read_f32()?,
            stream.read_f32()?,
            stream.read_f32()?,
          )),
          19 => Variant::Color(Color::rgba(
            stream.read_f32()?,
            stream.read_f32()?,
            stream.read_f32()?,
            stream.read_f32()?,
          )),
          20 => Variant::Color32(Color32::rgba(
            stream.read_u8()?,
            stream.read_u8()?,
            stream.read_u8()?,
            stream.read_u8()?,
          )),
          _ => return Err(StreamError::InvalidData),
        };

        Ok(Chunk::Variant(variant))
      }
      1 => {
        let len = stream.read_u32()?;
        let mut sequence = Vec::with_capacity(len as usize);

        for _ in 0..len {
          sequence.push(self.read_chunk(stream)?);
        }

        Ok(Chunk::Sequence(sequence))
      }
      2 => {
        let len = stream.read_u32()?;
        let mut map = FastHashMap::default();

        for _ in 0..len {
          let key = stream.read_string()?;
          let value = self.read_chunk(stream)?;

          map.insert(key, value);
        }

        Ok(Chunk::Map(map))
      }
      _ => Err(StreamError::InvalidData),
    }
  }

  fn write_chunk(&mut self, stream: &mut dyn OutputStream, chunk: &Chunk) -> Result<(), StreamError> {
    match chunk {
      Chunk::Variant(variant) => {
        stream.write_u8(0)?;

        match variant {
          Variant::Null => {
            stream.write_u8(0)?;
          }
          Variant::Bool(value) => {
            stream.write_u8(1)?;
            stream.write_u8(*value as u8)?
          }
          Variant::Char(value) => {
            stream.write_u8(2)?;
            stream.write_u8(*value as u8)?
          }
          Variant::U8(value) => {
            stream.write_u8(3)?;
            stream.write_u8(*value)?
          }
          Variant::U16(value) => {
            stream.write_u8(4)?;
            stream.write_u16(*value)?
          }
          Variant::U32(value) => {
            stream.write_u8(5)?;
            stream.write_u32(*value)?
          }
          Variant::U64(value) => {
            stream.write_u8(6)?;
            stream.write_u64(*value)?
          }
          Variant::I8(value) => {
            stream.write_u8(7)?;
            stream.write_i8(*value)?
          }
          Variant::I16(value) => {
            stream.write_u8(8)?;
            stream.write_i16(*value)?
          }
          Variant::I32(value) => {
            stream.write_u8(9)?;
            stream.write_i32(*value)?
          }
          Variant::I64(value) => {
            stream.write_u8(10)?;
            stream.write_i64(*value)?
          }
          Variant::F32(value) => {
            stream.write_u8(11)?;
            stream.write_f32(*value)?
          }
          Variant::F64(value) => {
            stream.write_u8(12)?;
            stream.write_f64(*value)?
          }
          Variant::String(value) => {
            stream.write_u8(13)?;
            stream.write_string(value)?
          }
          Variant::StringName(value) => {
            stream.write_u8(14)?;
            stream.write_string(&value.to_string())?
          }
          Variant::Vec2(value) => {
            stream.write_u8(15)?;
            stream.write_f32(value.x)?;
            stream.write_f32(value.y)?;
          }
          Variant::Vec3(value) => {
            stream.write_u8(16)?;
            stream.write_f32(value.x)?;
            stream.write_f32(value.y)?;
            stream.write_f32(value.z)?;
          }
          Variant::Vec4(value) => {
            stream.write_u8(17)?;
            stream.write_f32(value.x)?;
            stream.write_f32(value.y)?;
            stream.write_f32(value.z)?;
            stream.write_f32(value.w)?;
          }
          Variant::Quat(value) => {
            stream.write_u8(18)?;
            stream.write_f32(value.x)?;
            stream.write_f32(value.y)?;
            stream.write_f32(value.z)?;
            stream.write_f32(value.w)?;
          }
          Variant::Color(value) => {
            stream.write_u8(19)?;
            stream.write_f32(value.r)?;
            stream.write_f32(value.g)?;
            stream.write_f32(value.b)?;
            stream.write_f32(value.a)?;
          }
          Variant::Color32(value) => {
            stream.write_u8(20)?;
            stream.write_u8(value.r)?;
            stream.write_u8(value.g)?;
            stream.write_u8(value.b)?;
            stream.write_u8(value.a)?;
          }
          Variant::Pointer(_) => todo!(),
          Variant::Callable(_) => todo!(),
          Variant::Any(_) => todo!(),
        }
      }
      Chunk::Sequence(sequence) => {
        stream.write_u8(1)?;
        stream.write_u32(sequence.len() as u32)?;

        for chunk in sequence {
          self.write_chunk(stream, chunk)?;
        }
      }
      Chunk::Map(map) => {
        stream.write_u8(2)?;
        stream.write_u32(map.len() as u32)?;

        for (key, value) in map {
          stream.write_string(key)?;

          self.write_chunk(stream, value)?;
        }
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use macros::{Deserialize, Serialize};

  use super::*;
  use crate::{Color32, Vec3};

  /// A test struct for serialization/deserialization.
  #[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
  struct TestStruct {
    value_1: u32,
    value_2: f64,
    value_3: String,
    value_4: NestedStruct,
  }

  /// A nested struct for serialization/deserialization.
  #[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
  struct NestedStruct {
    value_1: Vec3,
    value_2: Color32,
  }

  #[test]
  fn it_should_serialize_basic_data_to_and_from_binary() {
    let input = TestStruct {
      value_1: 42,
      value_2: std::f64::consts::PI,
      value_3: "Hello, World!".to_string(),
      value_4: NestedStruct {
        value_1: Vec3::new(0., 1., 2.),
        value_2: Color32::rgb(255, 0, 255),
      },
    };

    let bytes = input.to_binary_bytes().unwrap();
    let output = TestStruct::from_binary_bytes(&bytes).unwrap();

    assert_eq!(input, output);
  }
}
