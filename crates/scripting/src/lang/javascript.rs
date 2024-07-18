//! JavaScript scripting language support.

use common::Variant;
use quick_js::{Arguments, Context, JsValue};

use super::*;

/// A runtime for executing Javascript scripts.
pub struct JavaScriptRuntime {
  context: Context,
}

impl JavaScriptRuntime {
  pub fn new() -> Self {
    Self {
      context: Context::new().unwrap(),
    }
  }
}

impl ScriptRuntime for JavaScriptRuntime {
  fn eval(&self, code: &str) -> Result<ScriptValue, ScriptError> {
    self
      .context
      .eval(code)
      .map(|it| it.into())
      .map_err(|it| ScriptError::ExecutionError(it.to_string()))
  }

  fn add_callback<F>(&mut self, name: &str, callback: impl ScriptCallback<F> + 'static) {
    self
      .context
      .add_callback(name, move |args: Arguments| {
        // convert arguments
        let args = args.into_vec().iter().map(|it| it.clone().into()).collect::<Vec<_>>();

        // convert result
        callback
          .call(&args)
          .map(JsValue::from)
          .map_err(|it| format!("{:?}", it))
      })
      .unwrap();
  }
}

impl From<JsValue> for ScriptValue {
  fn from(value: JsValue) -> Self {
    match value {
      JsValue::Undefined => ScriptValue::new(Variant::Null),
      JsValue::Null => ScriptValue::new(Variant::Null),
      JsValue::Bool(value) => ScriptValue::new(Variant::Bool(value)),
      JsValue::Int(value) => ScriptValue::new(Variant::I32(value)),
      JsValue::Float(value) => ScriptValue::new(Variant::F64(value)),
      JsValue::String(value) => ScriptValue::new(Variant::String(value.clone())),
      JsValue::Array(_) => todo!("Array conversion not implemented"),
      JsValue::Object(_) => todo!("Object conversion not implemented"),
      _ => panic!(
        "Unsupported
JsValue type"
      ),
    }
  }
}

impl From<ScriptValue> for JsValue {
  fn from(value: ScriptValue) -> Self {
    match value.as_variant() {
      Variant::Null => JsValue::Null,
      Variant::Bool(value) => JsValue::Bool(*value),
      Variant::Char(value) => JsValue::String(value.to_string()),
      Variant::U8(value) => JsValue::Int(*value as i32),
      Variant::U16(value) => JsValue::Int(*value as i32),
      Variant::U32(value) => JsValue::Int(*value as i32),
      Variant::U64(value) => JsValue::Int(*value as i32),
      Variant::I8(value) => JsValue::Int(*value as i32),
      Variant::I16(value) => JsValue::Int(*value as i32),
      Variant::I32(value) => JsValue::Int(*value),
      Variant::I64(value) => JsValue::Int(*value as i32),
      Variant::F32(value) => JsValue::Float(*value as f64),
      Variant::F64(value) => JsValue::Float(*value),
      Variant::String(value) => JsValue::String(value.clone()),
      Variant::StringName(value) => JsValue::String(value.to_string()),
      Variant::Vec2(value) => JsValue::Array(vec![JsValue::Float(value.x as f64), JsValue::Float(value.y as f64)]),
      Variant::Vec3(value) => JsValue::Array(vec![
        JsValue::Float(value.x as f64),
        JsValue::Float(value.y as f64),
        JsValue::Float(value.z as f64),
      ]),
      Variant::Vec4(value) => JsValue::Array(vec![
        JsValue::Float(value.x as f64),
        JsValue::Float(value.y as f64),
        JsValue::Float(value.z as f64),
        JsValue::Float(value.w as f64),
      ]),
      Variant::Quat(value) => JsValue::Array(vec![
        JsValue::Float(value.x as f64),
        JsValue::Float(value.y as f64),
        JsValue::Float(value.z as f64),
        JsValue::Float(value.w as f64),
      ]),
      Variant::Color(value) => JsValue::Array(vec![
        JsValue::Float(value.r as f64),
        JsValue::Float(value.g as f64),
        JsValue::Float(value.b as f64),
        JsValue::Float(value.a as f64),
      ]),
      Variant::Color32(value) => JsValue::Array(vec![
        JsValue::Int(value.r as i32),
        JsValue::Int(value.g as i32),
        JsValue::Int(value.b as i32),
        JsValue::Int(value.a as i32),
      ]),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_javascript_evaluation() {
    let mut runtime = JavaScriptRuntime::new();

    runtime.add_callback("add", |x: i32, y: i32| x + y);

    let result = runtime.eval("add(32.0, 42)").unwrap();

    assert_eq!(result, ScriptValue::new(Variant::I32(74)));
  }
}
