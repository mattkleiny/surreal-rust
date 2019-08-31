//! Asset management system.

pub enum Asset<T> {
  Ready(T),
  NotReady,
}