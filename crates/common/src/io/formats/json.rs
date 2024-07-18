use super::*;
use crate::io::formats::json::parser::JsonToken;

/// A file format for working with JSON.
#[derive(Default)]
pub struct JsonFileFormat {
  indent: usize,
}

impl FileFormat for JsonFileFormat {
  fn read_chunk(&mut self, stream: &mut dyn InputStream) -> Result<Chunk, StreamError> {
    let mut reader = parser::JsonStreamReader::new(stream);

    while let Ok(token) = reader.next_token() {
      // TODO: do something with the token
      match token {
        JsonToken::ObjectStart => {}
        JsonToken::ObjectEnd => {}
        JsonToken::ArrayStart => {}
        JsonToken::ArrayEnd => {}
        JsonToken::String(_) => {}
        JsonToken::Number(_) => {}
        JsonToken::Boolean(_) => {}
        JsonToken::Null => {}
      }
    }

    todo!()
  }

  fn write_chunk(&mut self, stream: &mut dyn OutputStream, chunk: &Chunk) -> Result<(), StreamError> {
    match chunk {
      Chunk::Variant(variant) => match variant {
        Variant::Null => stream.write_string("null")?,
        Variant::Bool(value) => stream.write_string(&value.to_string())?,
        Variant::Char(value) => stream.write_string(&format!("\"{}\"", value))?,
        Variant::U8(value) => stream.write_string(&value.to_string())?,
        Variant::U16(value) => stream.write_string(&value.to_string())?,
        Variant::U32(value) => stream.write_string(&value.to_string())?,
        Variant::U64(value) => stream.write_string(&value.to_string())?,
        Variant::I8(value) => stream.write_string(&value.to_string())?,
        Variant::I16(value) => stream.write_string(&value.to_string())?,
        Variant::I32(value) => stream.write_string(&value.to_string())?,
        Variant::I64(value) => stream.write_string(&value.to_string())?,
        Variant::F32(value) => stream.write_string(&value.to_string())?,
        Variant::F64(value) => stream.write_string(&value.to_string())?,
        Variant::String(value) => stream.write_string(&format!("\"{}\"", value))?,
        Variant::StringName(value) => stream.write_string(&format!("\"{}\"", value))?,
        Variant::Vec2(value) => stream.write_string(&format!("[{}, {}]", value.x, value.y))?,
        Variant::Vec3(value) => stream.write_string(&format!("[{}, {}, {}]", value.x, value.y, value.z))?,
        Variant::Vec4(value) => {
          stream.write_string(&format!("[{}, {}, {}, {}]", value.x, value.y, value.z, value.w))?
        }
        Variant::Quat(value) => {
          stream.write_string(&format!("[{}, {}, {}, {}]", value.x, value.y, value.z, value.w))?
        }
        Variant::Color(value) => {
          stream.write_string(&format!("[{}, {}, {}, {}]", value.r, value.g, value.b, value.a))?;
        }
        Variant::Color32(value) => {
          stream.write_string(&format!("[{}, {}, {}, {}]", value.r, value.g, value.b, value.a))?;
        }
      },
      Chunk::Sequence(sequence) => {
        stream.write_string("[")?;

        for (i, value) in sequence.iter().enumerate() {
          if i > 0 {
            stream.write_string(",")?;
          }

          self.write_chunk(stream, value)?;
        }

        stream.write_string("]")?;
      }
      Chunk::Map(map) => {
        stream.write_string("{")?;

        self.indent += 1;

        for (i, (key, value)) in map.iter().enumerate() {
          if i > 0 {
            stream.write_string(",")?;
          }

          stream.write_string("\n")?;
          stream.write_string(&"  ".repeat(self.indent))?;
          stream.write_string(&format!("\"{}\": ", key))?;

          self.write_chunk(stream, value)?;
        }

        self.indent -= 1;

        stream.write_string("\n")?;
        stream.write_string(&"  ".repeat(self.indent))?;
        stream.write_string("}")?;
      }
    }

    Ok(())
  }
}

#[allow(dead_code)]
mod parser {
  use super::*;

  /// A token in the JSON stream.
  #[derive(Debug, PartialEq)]
  pub enum JsonToken {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
  }

  /// Current state of the JSON parser.
  #[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
  pub enum JsonState {
    #[default]
    Start,
    ObjectStart,
    ObjectKey,
    ObjectValue,
    ObjectEnd,
    ArrayStart,
    ArrayValue,
    ArrayEnd,
    String,
    Number,
    Boolean,
    Null,
    End,
  }

  /// A parser for reading data from a JSON stream.
  ///
  /// This is a high-performance JSON parser that reads data from a stream in a
  /// forward-only fashion, one token at a time.
  pub struct JsonStreamReader<'a> {
    stream: &'a mut dyn InputStream,
    state: JsonState,
  }

  impl<'a> JsonStreamReader<'a> {
    /// Creates a new JSON parser for the given input stream.
    pub fn new(stream: &'a mut dyn InputStream) -> Self {
      Self {
        stream,
        state: JsonState::Start,
      }
    }

    /// Returns the current state of the JSON parser.
    pub fn state(&self) -> JsonState {
      self.state
    }

    /// Reads the next token from the JSON stream.
    pub fn next_token(&mut self) -> Result<JsonToken, StreamError> {
      self.stream.skip_whitespace()?;

      while let Ok(next) = self.stream.read_char() {
        match next {
          '{' => return Ok(JsonToken::ObjectStart),
          '}' => return Ok(JsonToken::ObjectEnd),
          '[' => return Ok(JsonToken::ArrayStart),
          ']' => return Ok(JsonToken::ArrayEnd),
          '"' => {
            let mut string = String::new();

            while let Ok(next) = self.stream.read_char() {
              match next {
                '"' => return Ok(JsonToken::String(string)),
                _ => string.push(next),
              }
            }
          }
          _ => {}
        }
      }

      Err(StreamError::EndOfStream)
    }
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
  fn it_should_serialize_basic_data_to_json() {
    let input = TestStruct {
      value_1: 42,
      value_2: std::f64::consts::PI,
      value_3: "Hello, World!".to_string(),
      value_4: NestedStruct {
        value_1: Vec3::new(0., 1., 2.),
        value_2: Color32::rgb(255, 0, 255),
      },
    };

    let json = input.to_json_string().unwrap();

    println!("{}", json);
  }
}
