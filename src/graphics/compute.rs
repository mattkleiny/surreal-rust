//! Compute shader abstractions for the engine.
//!
//! Compute programs allow for the execution of arbitrary code on the GPU.

/// Indicates the kinds of barriers that can be synchronized in the GPU compute system.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ComputeBarrier {
  ImageAccess,
}
