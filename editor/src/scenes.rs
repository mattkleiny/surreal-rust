//! Scene management abstractions and tools

use core::collections::Arena;
use core::maths::Guid;

/// Describes a game scene that can be simulated.
pub struct Scene {
  // resources: Arena<Resource>,
  entities: Arena<Entity>,
}

/// Describes a scene, de/serializable to/from disk.
#[derive(Default, Serialize, Deserialize)]
pub struct SceneDescriptor {
  resources: Vec<ResourceDescriptor>,
  entities: Vec<EntityDescriptor>,
}

// /// A resource in the scene.
// pub struct Resource {
//   id: Guid,
// }

/// Describes a resource, de/serializable to/from disk.
#[derive(Default, Serialize, Deserialize)]
struct ResourceDescriptor {
  id: Guid,
}

/// An entity in the scene.
pub struct Entity {
  id: Guid,
  name: String,
  enabled: bool,
  visible: bool,
}

/// Describes an entity, de/serializable to/from disk.
#[derive(Default, Serialize, Deserialize)]
struct EntityDescriptor {
  id: Guid,
  name: String,
  enabled: bool,
  visible: bool,
  components: Vec<ComponentDescriptor>,
}

/// Describes an entity component, de/serializable to/from disk.
#[derive(Default, Serialize, Deserialize)]
struct ComponentDescriptor {
  kind: Guid,
}

impl Default for Scene {
  fn default() -> Self {
    Scene::from(SceneDescriptor::default())
  }
}

impl From<SceneDescriptor> for Scene {
  fn from(descriptor: SceneDescriptor) -> Self {
    Scene::from(&descriptor)
  }
}

impl From<&SceneDescriptor> for Scene {
  fn from(descriptor: &SceneDescriptor) -> Self {
    let mut scene = Scene {
      // resources: Arena::with_capacity(descriptor.resources.len()),
      entities: Arena::with_capacity(descriptor.entities.len()),
    };

    // for descriptor in &descriptor.resources {
    //   scene.resources.add(Resource {
    //     id: descriptor.id
    //   });
    // }

    for descriptor in &descriptor.entities {
      scene.entities.add(Entity {
        id: descriptor.id,
        name: descriptor.name.clone(),
        enabled: descriptor.enabled,
        visible: descriptor.visible,
      });
    }

    scene
  }
}

impl Into<SceneDescriptor> for Scene {
  fn into(self) -> SceneDescriptor {
    (&self).into()
  }
}

impl Into<SceneDescriptor> for &Scene {
  fn into(self) -> SceneDescriptor {
    let mut descriptor = SceneDescriptor::default();

    // for (_, resource) in self.resources.iter() {
    //   descriptor.resources.push(ResourceDescriptor { id: resource.id })
    // }

    for (_, entity) in self.entities.iter() {
      descriptor.entities.push(EntityDescriptor {
        id: entity.id,
        name: entity.name.clone(),
        enabled: entity.enabled,
        visible: entity.visible,
        components: vec![], // TODO: components
      })
    }

    descriptor
  }
}

#[cfg(test)]
mod tests {
  use core::maths::FromRandom;

  use super::*;

  #[test]
  fn it_should_serialize_to_disk() {
    let descriptor = SceneDescriptor {
      resources: vec![],
      entities: vec![EntityDescriptor {
        id: Guid::random(),
        name: "Player".to_string(),
        enabled: true,
        visible: true,
        components: vec![
          ComponentDescriptor { kind: Guid::random() },
          ComponentDescriptor { kind: Guid::random() },
        ],
      }],
    };

    let result = serde_json::to_string(&descriptor).unwrap();

    println!("{:?}", result);
  }
}
