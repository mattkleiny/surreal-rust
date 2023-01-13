//! A headless audio backend for testing and etc.

use super::*;

/// A headless [`AudioBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
#[derive(Default)]
pub struct HeadlessBackend;

impl AudioBackend for HeadlessBackend {}
