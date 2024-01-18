use std::fmt::Debug;

use super::{SceneContext, SceneEvent, SceneRenderContext};

/// Represents a component in a scene.
///
/// Components are attached to [`SceneNode`]s. They receive callbacks in
/// response to scene lifecycle events, and can access and mutate data from
/// their parent node.
///
/// # Examples
///
/// ```
/// use surreal_scene::{SceneComponent, SceneContext, SceneEvent};
///
/// struct TestComponent {}
///
/// impl SceneComponent for TestComponent {
///   fn on_update(&mut self, context: SceneContext, delta_time: f32) {
///     println!("TestComponent::on_update");
///   }
/// }
///
/// let mut component = TestComponent {};
///
/// component.notify(
///   SceneContext::default(),
///   &mut SceneEvent::Update(0.16)
/// );
/// ```
#[allow(unused_variables)]
pub trait SceneComponent {
  /// Returns a friendly name for this component, for debugging/editor/etc.
  fn name(&self) -> &str {
    let name = std::any::type_name::<Self>();

    name.rsplit_once("::").map(|split| split.1).unwrap_or(name)
  }

  /// Notifies of an incoming [`SceneEvent`] to the graph or sub-graph.
  fn notify(&mut self, context: SceneContext, event: &mut SceneEvent) {
    match event {
      SceneEvent::Awake => self.on_awake(context),
      SceneEvent::Start => self.on_start(context),
      SceneEvent::Enable => self.on_enable(context),
      SceneEvent::Disable => self.on_disable(context),
      SceneEvent::Destroy => self.on_destroy(context),
      SceneEvent::Update(delta_time) => self.on_update(context, *delta_time),
      SceneEvent::Draw(render_context) => self.on_draw(context, render_context),
    }
  }

  fn on_awake(&mut self, context: SceneContext) {}
  fn on_start(&mut self, context: SceneContext) {}
  fn on_enable(&mut self, context: SceneContext) {}
  fn on_disable(&mut self, context: SceneContext) {}
  fn on_destroy(&mut self, context: SceneContext) {}
  fn on_update(&mut self, context: SceneContext, delta_time: f32) {}
  fn on_draw(&mut self, context: SceneContext, render_context: &mut SceneRenderContext) {}
}

/// A set of [`SceneComponent`]s in a [`SceneNode`].
///
/// This is a simple wrapper around a [`Vec`] of [`Box`]ed [`SceneComponent`]s.
/// It provides a few convenience methods for working with the set.
///
/// # Examples
///
/// ```
/// use surreal_scene::{SceneComponent, SceneComponentSet};
///
/// struct Foo;
/// impl SceneComponent for Foo {}
///
/// struct Bar;
/// impl SceneComponent for Bar {}
///
/// let mut set = SceneComponentSet::default();
///
/// set.push(Foo);
/// set.push(Bar);
///
/// assert_eq!(set.iter().count(), 2);
/// ```
#[derive(Default)]
pub struct SceneComponentSet {
  // TODO: hierarchical bit mask over ComponentKind?
  components: Vec<Box<dyn SceneComponent>>,
}

impl SceneComponentSet {
  /// Builds a [`SceneComponentSet`] from the given array.
  pub fn from_array<const S: usize>(components: [Box<dyn SceneComponent>; S]) -> Self {
    Self {
      components: Vec::from(components),
    }
  }

  /// Adds a new [`SceneComponent`] to the set.
  pub fn push<C: SceneComponent + 'static>(&mut self, component: C) {
    self.components.push(Box::new(component));
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
      fn on_awake(&mut self, _context: SceneContext) {
        self.awake = true;
      }

      fn on_start(&mut self, _context: SceneContext) {
        self.start = true;
      }

      fn on_enable(&mut self, _context: SceneContext) {
        self.enable = true;
      }

      fn on_disable(&mut self, _context: SceneContext) {
        self.disable = true;
      }

      fn on_destroy(&mut self, _context: SceneContext) {
        self.destroy = true;
      }

      fn on_update(&mut self, _context: SceneContext, _delta_time: f32) {
        self.update = true;
      }

      fn on_draw(&mut self, _context: SceneContext, _render_context: &mut SceneRenderContext) {
        self.draw = true;
      }
    }

    let mut component = TestComponent::default();

    component.notify(SceneContext::default(), &mut SceneEvent::Awake);
    assert!(component.awake);

    component.notify(SceneContext::default(), &mut SceneEvent::Start);
    assert!(component.start);

    component.notify(SceneContext::default(), &mut SceneEvent::Enable);
    assert!(component.enable);

    component.notify(SceneContext::default(), &mut SceneEvent::Disable);
    assert!(component.disable);

    component.notify(SceneContext::default(), &mut SceneEvent::Destroy);
    assert!(component.destroy);

    component.notify(SceneContext::default(), &mut SceneEvent::Update(0.16));
    assert!(component.update);

    component.notify(
      SceneContext::default(),
      &mut SceneEvent::Draw(&mut SceneRenderContext::default()),
    );
    assert!(component.draw);
  }

  #[test]
  fn component_set_should_push_new_entries() {
    struct TestComponent1 {}
    struct TestComponent2 {}

    impl SceneComponent for TestComponent1 {}
    impl SceneComponent for TestComponent2 {}

    let mut set = SceneComponentSet::default();

    set.push(TestComponent1 {});
    set.push(TestComponent2 {});

    assert_eq!(set.iter().count(), 2);
  }

  #[test]
  fn component_set_should_print_helpful_debug() {
    struct TestComponent1 {}
    struct TestComponent2 {}

    impl SceneComponent for TestComponent1 {}
    impl SceneComponent for TestComponent2 {}

    let mut set = SceneComponentSet::default();

    set.push(TestComponent1 {});
    set.push(TestComponent2 {});

    assert_eq!(
      format!("{:?}", set),
      "[\"TestComponent1\", \"TestComponent2\"]"
    );
  }
}
