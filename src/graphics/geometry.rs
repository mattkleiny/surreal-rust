/// A fast and lightweight geometry batch renderer.
///
/// This batch pre-allocates an array of vertices and re-uses it to tessellate shapes and polygons.
///
/// Batching is possible over 1 material and texture pair; each texture swap requires a flush
/// and so it's important to pre-sort sprites into batches by material and texture.
pub struct GeometryBatch {
  // TODO: implement me
}