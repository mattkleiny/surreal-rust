//! A virtual file system.

// TODO: play with this some more.

/// Represents a path in a virtual file system.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path(String);