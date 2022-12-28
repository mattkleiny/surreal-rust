//! A module for streaming assets and resources for use in larger games.
//!
//! This module provides a streaming API for loading assets and resources from
//! disk.

#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

/// A top-level manager for the streaming system.
///
/// The manager receives a series of [`StreamingRequest`]s from components, such
/// as the [`StreamingArea`] and [`StreamingVolume`]. These request are high-level
/// indications of the kinds of manifests we need to load from the asset system.
///
/// Each [`StreamingRequest`] is then associated with a unique [`StreamingJob`]
/// and prioritized in the queue. The queue is drained by background workers that
/// can be reconfigured depending on platform and the number of cores available.
///
/// In order to actualize [`StreamingRequest`] changes in the system, we
/// delegate to a [`StreamingHandler`] who is then responsible for loading assets
/// and unloading old assets.
pub struct StreamingManager {}

/// A handler for [`StreamingRequest`]s.
///
/// The handler is responsible for loading and unloading assets based on the
/// requests it receives from the [`StreamingManager`].
pub trait StreamingHandler {
  async fn handle(&self, request: &StreamingRequest) -> StreamingResponse;
}

/// A single job for the [`StreamingManager`].
pub struct StreamingJob {}

/// A handle for a [`StreamingJob`].
pub struct StreamingHandle {}

/// A request to load a bundle of resources in the [`StreamingManager`].
pub struct StreamingRequest {}

/// The response for a [`StreamingHandler`] request.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StreamingResponse {
  Success,
  Failure,
}

/// A 2d area that can indicate to the [`StreamingManager`] to change resources.
pub trait StreamingArea {}

/// A 3d volume that can indicate to the [`StreamingManager`] to change resources.
pub trait StreamingVolume {}
