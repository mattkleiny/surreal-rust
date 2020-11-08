/// Encodes the given string with a simple run-length encoding scheme.
pub fn run_length_encode(raw: impl AsRef<str>) -> String {
  fn convert(n: u32) -> String {
    match n {
      1 => "".into(),
      _ => n.to_string(),
    }
  }

  if raw.as_ref().is_empty() {
    return raw.as_ref().into();
  }

  let mut encoded = String::new();
  let mut count = 1;
  let mut current = raw.as_ref().chars().nth(0).unwrap();

  for character in raw.as_ref().chars().skip(1) {
    if character == current {
      count += 1;
    } else {
      encoded.push_str(&convert(count));
      encoded.push(current);
      current = character;
      count = 1;
    }
  }

  encoded.push_str(&convert(count));
  encoded.push(current);

  encoded
}

/// Decodes the given string with a simple run-length encoding scheme.
fn run_length_decode(raw: impl AsRef<str>) -> String {
  let mut numerals = String::new();
  let mut decoded = String::new();

  for character in raw.as_ref().chars() {
    if character.is_numeric() {
      numerals.push(character);
    } else {
      let num: usize = numerals.parse().unwrap_or(1);

      decoded.push_str(&character.to_string().repeat(num));
      numerals.clear();
    }
  }

  decoded
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_encode_and_decode() {
    let raw = "i aaaaam a a teeeexxxxxttt messssaaaagggeee";
    let encoded = run_length_encode(raw);
    let decoded = run_length_decode(encoded.as_str());

    assert_ne!(encoded, decoded);
    assert_eq!(decoded, raw);
  }
}
