use std::{
  any::{Any, TypeId},
  fmt::Debug,
};

use super::SceneEvent;

/// Represents a component in a scene.
///
/// Components are attached to [`SceneNode`]s. They receive callbacks in
/// response to scene lifecycle events, and can access and mutate data from
/// their parent node.
#[allow(unused_variables)]
pub trait SceneComponent {
  /// Returns a friendly name for this component, for debugging/editor/etc.
  fn name(&self) -> &str {
    let name = std::any::type_name::<Self>();

    name.rsplit_once("::").map(|split| split.1).unwrap_or(name)
  }

  /// Notifies of an incoming [`SceneEvent`] to the graph or sub-graph.
  fn notify(&mut self, event: &mut SceneEvent) {
    match event {
      SceneEvent::Awake => self.on_awake(),
      SceneEvent::Start => self.on_start(),
      SceneEvent::Enable => self.on_enable(),
      SceneEvent::Disable => self.on_disable(),
      SceneEvent::Destroy => self.on_destroy(),
      SceneEvent::Update(delta_time) => self.on_update(*delta_time),
      SceneEvent::Render(context) => self.on_draw(context),
      SceneEvent::TransformChanged => self.on_transform_changed(),
    }
  }

  fn on_awake(&mut self) {}
  fn on_start(&mut self) {}
  fn on_enable(&mut self) {}
  fn on_disable(&mut self) {}
  fn on_destroy(&mut self) {}
  fn on_update(&mut self, delta_time: f32) {}
  fn on_draw(&mut self, render_context: &mut graphics::Renderer) {}
  fn on_transform_changed(&mut self) {}
}

/// A set of [`SceneComponent`]s in a [`SceneNode`].
///
/// This is a simple wrapper around a [`Vec`] of [`Box`]ed [`SceneComponent`]s.
/// It provides a few convenience methods for working with the set.
#[derive(Default)]
pub struct SceneComponentSet {
  components: Vec<Box<dyn SceneComponent>>,
}

impl SceneComponentSet {
  /// Builds a [`SceneComponentSet`] from the given array.
  pub fn from_array<const S: usize>(components: [Box<dyn SceneComponent>; S]) -> Self {
    Self {
      components: Vec::from(components),
    }
  }

  /// Returns `true` if this set contains no [`SceneComponent`]s.
  pub fn is_empty(&self) -> bool {
    self.components.is_empty()
  }

  /// Returns the number of [`SceneComponent`]s in this set.
  pub fn len(&self) -> usize {
    self.components.len()
  }

  /// Gets the [`SceneComponent`] of the given type in this set.
  pub fn get<C: SceneComponent + 'static>(&self) -> Option<&C> {
    for component in &self.components {
      if TypeId::of::<C>() == component.type_id() {
        return Some(unsafe { &*(component.as_ref() as *const dyn SceneComponent as *const C) });
      }
    }

    None
  }

  /// Mutably gets the [`SceneComponent`] of the given type in this set.
  pub fn get_mut<C: SceneComponent + 'static>(&mut self) -> Option<&mut C> {
    // TODO: fix this
    // for component in &mut self.components {
    //   if TypeId::of::<C>() == component.type_id() {
    //     return Some(unsafe { &mut *(component.as_mut() as *mut dyn SceneComponent
    // as *mut C) });   }
    // }

    None
  }

  /// Adds a new [`SceneComponent`] to the set.
  pub fn add<C: SceneComponent + 'static>(&mut self, component: C) {
    self.components.push(Box::new(component));
  }

  /// Removes the [`SceneComponent`] of the given type from this set.
  pub fn remove<C: SceneComponent + 'static>(&mut self) -> common::Result<()> {
    for (index, component) in self.components.iter().enumerate() {
      if TypeId::of::<C>() == component.type_id() {
        self.components.remove(index);
        return Ok(());
      }
    }

    return Err(common::anyhow!("Component not found"));
  }

  /// Iterates the [`SceneComponent`]s in this set.
  pub fn iter(&self) -> impl Iterator<Item = &Box<dyn SceneComponent>> {
    self.components.iter()
  }

  /// Mutably iterates the [`SceneComponent`]s in this set.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn SceneComponent>> {
    self.components.iter_mut()
  }
}

impl Debug for SceneComponentSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_list()
      .entries(self.components.iter().map(|c| c.name()))
      .finish()
  }
}

impl<'a> IntoIterator for &'a SceneComponentSet {
  type Item = &'a Box<dyn SceneComponent>;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut SceneComponentSet {
  type Item = &'a mut Box<dyn SceneComponent>;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn component_should_have_reasonable_default_name() {
    struct TestComponent1 {}
    struct TestComponent2 {}

    impl SceneComponent for TestComponent1 {}
    impl SceneComponent for TestComponent2 {}

    let component1 = TestComponent1 {};
    let component2 = TestComponent2 {};

    assert_eq!(component1.name(), "TestComponent1");
    assert_eq!(component2.name(), "TestComponent2");
  }

  #[test]
  fn components_should_dispatch_to_helper() {
    #[derive(Default)]
    struct TestComponent {
      awake: bool,
      start: bool,
      enable: bool,
      disable: bool,
      destroy: bool,
      update: bool,
      draw: bool,
    }

    impl SceneComponent for TestComponent {
      fn on_awake(&mut self) {
        self.awake = true;
      }

      fn on_start(&mut self) {
        self.start = true;
      }

      fn on_enable(&mut self) {
        self.enable = true;
      }

      fn on_disable(&mut self) {
        self.disable = true;
      }

      fn on_destroy(&mut self) {
        self.destroy = true;
      }

      fn on_update(&mut self, _delta_time: f32) {
        self.update = true;
      }

      fn on_draw(&mut self, _render_context: &mut graphics::Renderer) {
        self.draw = true;
      }
    }

    let mut component = TestComponent::default();

    component.notify(&mut SceneEvent::Awake);
    assert!(component.awake);

    component.notify(&mut SceneEvent::Start);
    assert!(component.start);

    component.notify(&mut SceneEvent::Enable);
    assert!(component.enable);

    component.notify(&mut SceneEvent::Disable);
    assert!(component.disable);

    component.notify(&mut SceneEvent::Destroy);
    assert!(component.destroy);

    component.notify(&mut SceneEvent::Update(0.16));
    assert!(component.update);
  }

  #[test]
  fn component_set_should_push_new_entries() {
    struct TestComponent1 {}
    struct TestComponent2 {}

    impl SceneComponent for TestComponent1 {}
    impl SceneComponent for TestComponent2 {}

    let mut set = SceneComponentSet::default();

    set.add(TestComponent1 {});
    set.add(TestComponent2 {});

    assert_eq!(set.iter().count(), 2);
  }

  #[test]
  fn component_set_should_print_helpful_debug() {
    struct TestComponent1 {}
    struct TestComponent2 {}

    impl SceneComponent for TestComponent1 {}
    impl SceneComponent for TestComponent2 {}

    let mut set = SceneComponentSet::default();

    set.add(TestComponent1 {});
    set.add(TestComponent2 {});

    assert_eq!(format!("{:?}", set), "[\"TestComponent1\", \"TestComponent2\"]");
  }
}
