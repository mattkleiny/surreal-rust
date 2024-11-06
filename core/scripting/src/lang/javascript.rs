use wasm_bindgen::prelude::*;
use js_sys::Function;
use common::Variant;

/// A JavaScript runtime for the Surreal scripting engine.
pub struct JavaScriptRuntime {
    context: js_sys::Object,
}

impl JavaScriptRuntime {
    /// Creates a new JavaScript runtime.
    pub fn new() -> Self {
        let context = js_sys::Object::new();
        Self { context }
    }

    /// Evaluates a JavaScript script.
    pub fn evaluate(&self, script: &str) -> Result<Variant, JsValue> {
        let result = js_sys::eval(script)?;
        Ok(self.convert_js_value_to_variant(result))
    }

    /// Converts a JavaScript value to a Variant.
    fn convert_js_value_to_variant(&self, value: JsValue) -> Variant {
        if value.is_undefined() {
            Variant::None
        } else if value.is_null() {
            Variant::None
        } else if value.is_boolean() {
            Variant::Bool(value.as_bool().unwrap())
        } else if value.is_number() {
            Variant::F64(value.as_f64().unwrap())
        } else if value.is_string() {
            Variant::String(value.as_string().unwrap())
        } else {
            Variant::None
        }
    }
}
