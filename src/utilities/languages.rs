/// Represents a language that can be parsed.
pub trait Language {
  type Statement;

  fn parse_statements(raw: impl AsRef<str>) -> Vec<Self::Statement>;
}