//! Allows using the physics engine from a scene.

use common::SceneComponent;

pub struct BoxCollider {}

impl SceneComponent for BoxCollider {}

pub struct SphereCollider {}

impl SceneComponent for SphereCollider {}

pub struct CapsuleCollider {}

impl SceneComponent for CapsuleCollider {}

pub struct MeshCollider {}

impl SceneComponent for MeshCollider {}

pub struct Rigidbody {}

impl SceneComponent for Rigidbody {}

pub struct WindEffector {}

impl SceneComponent for WindEffector {}

pub struct GravityEffector {}

impl SceneComponent for GravityEffector {}
