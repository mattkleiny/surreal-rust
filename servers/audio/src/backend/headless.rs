//! A headless audio backend for testing and etc.

use super::*;

/// A headless [`AudioServerBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
#[derive(Default)]
pub struct HeadlessBackend;

impl AudioServerBackend for HeadlessBackend {}