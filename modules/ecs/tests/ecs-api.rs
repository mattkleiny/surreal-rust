use surreal_ecs::{Component, World};

#[derive(Default)]
struct Health {}

impl Component for Health {}

#[derive(Default)]
struct Stamina {}

impl Component for Stamina {}

#[test]
fn ecs_api_should_be_easy_to_use() {
  let mut world = World::default();

  let entity1 = world.create_entity();
  let entity2 = world.create_entity();

  world.add_component(entity1, Health {});
  world.add_component(entity2, Stamina {});
}
