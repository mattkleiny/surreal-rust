//! A GDScript scripting language implementation for Surreal.

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "gdscript.pest"]
struct GDScriptParser;

#[cfg(test)]
mod tests {
  use pest::Parser;

  use super::*;

  #[test]
  fn it_should_parse_a_simple_expression() {
    let pairs = GDScriptParser::parse(Rule::ident_list, "a1 b2").unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
      println!("Rule: {:?}", pair.as_rule());
      println!("Span: {:?}", pair.as_span());
      println!("Text: {}", pair.as_str());

      for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
          Rule::alpha => println!("Letter: {}", inner_pair.as_str()),
          Rule::digit => println!("Digit: {}", inner_pair.as_str()),
          _ => unreachable!(),
        };
      }
    }
  }
}
