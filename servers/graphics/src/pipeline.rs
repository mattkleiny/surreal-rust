//! Rendering pipeline abstractions.

#[cfg(feature = "pipeline-highdef")]
pub mod highdef;
#[cfg(feature = "pipeline-universal")]
pub mod universal;
