//! Asset management abstractions.

use std::sync::Arc;

pub trait AssetReader<A> {}
pub trait AssetWriter<A> {}

#[derive(Clone, Debug)]
pub enum AssetRef<A> {
  Ready(Arc<A>),
  Loading,
  NotReady,
}

pub fn load_asset<A: AssetReader<A>, P: AsRef<str>>(_path: P) -> AssetRef<A> {
  unimplemented!()
}