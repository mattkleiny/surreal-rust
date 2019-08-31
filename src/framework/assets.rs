//! Asset management abstractions.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum AssetRef<A> {
  Ready(Arc<A>),
  Loading,
  NotReady,
}