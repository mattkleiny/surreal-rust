//! Lua bindings for internal types in Surreal.

use rlua::{MetaMethod, UserData, UserDataMethods};

use crate::maths::Vec2i;

/// Lua bindings for Vec2i.
impl UserData for Vec2i {
  fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
    methods.add_method("normal", |_, vec: &Vec2i, ()| { Ok(vec.x + vec.y) });
    methods.add_meta_function(MetaMethod::Add, |_, (vec1, vec2): (Vec2i, Vec2i)| {
      Ok(Vec2i::new(vec1.x + vec2.y, vec1.y + vec2.y))
    });
  }
}
