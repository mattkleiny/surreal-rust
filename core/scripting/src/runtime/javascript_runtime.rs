use rusty_v8 as v8;
use common::Variant;

/// A JavaScript runtime for the Surreal scripting engine.
pub struct JavaScriptRuntime {
    isolate: v8::OwnedIsolate,
    context: v8::Global<v8::Context>,
}

impl JavaScriptRuntime {
    /// Creates a new JavaScript runtime.
    pub fn new() -> Self {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = v8::Isolate::new(Default::default());
        let handle_scope = &mut v8::HandleScope::new(&isolate);
        let context = v8::Context::new(handle_scope);

        let global_context = v8::Global::new(handle_scope, context);

        Self {
            isolate,
            context: global_context,
        }
    }

    /// Evaluates a JavaScript script.
    pub fn evaluate(&self, script: &str) -> Result<Variant, String> {
        let handle_scope = &mut v8::HandleScope::new(&self.isolate);
        let context = v8::Local::new(handle_scope, &self.context);
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        let code = v8::String::new(scope, script).ok_or("Failed to create V8 string")?;
        let script = v8::Script::compile(scope, code, None).ok_or("Failed to compile script")?;
        let result = script.run(scope).ok_or("Failed to run script")?;

        Ok(self.convert_v8_value_to_variant(result))
    }

    /// Converts a V8 value to a Variant.
    fn convert_v8_value_to_variant(&self, value: v8::Local<v8::Value>) -> Variant {
        if value.is_undefined() {
            Variant::None
        } else if value.is_null() {
            Variant::None
        } else if value.is_boolean() {
            Variant::Bool(value.boolean_value(&self.isolate))
        } else if value.is_number() {
            Variant::F64(value.number_value(&self.isolate).unwrap())
        } else if value.is_string() {
            let v8_str = value.to_string(&self.isolate).unwrap();
            let rust_str = v8_str.to_rust_string_lossy(&self.isolate);
            Variant::String(rust_str)
        } else {
            Variant::None
        }
    }
}
