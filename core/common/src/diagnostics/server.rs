//! An IPC server for exposing diagnostic information to clients.
//!
//! This is primarily used for debugging and profiling, and is not intended to
//! be used in production.

use crate::TimeStamp;

/// An event that can be sent to a diagnostic listener.
#[derive(Debug)]
pub enum DiagnosticEvent {
  Log {
    level: super::LogLevel,
    message: String,
  },
  Profile {
    name: String,
    start: TimeStamp,
    end: TimeStamp,
  },
}

/// A listener that can receive diagnostic events.
pub trait DiagnosticListener {
  fn on_debug_event(&mut self, event: &DiagnosticEvent);
}

/// Allows a closure to be used as a diagnostic listener.
impl<F: FnMut(&DiagnosticEvent)> DiagnosticListener for F {
  fn on_debug_event(&mut self, event: &DiagnosticEvent) {
    self(event)
  }
}

/// A potential error that can occur when starting a diagnostic server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticError {
  FailedToStart,
  FailedToConnect,
  FailedToSend,
}

/// A server that exposes diagnostic information to clients.
pub struct DiagnosticServer {}

/// A client that can connect to a diagnostic server.
pub struct DiagnosticClient {
  _listeners: Vec<Box<dyn DiagnosticListener>>,
}

impl DiagnosticClient {
  /// Adds a listener to the client.
  pub fn add_listener(&mut self, listener: impl DiagnosticListener + 'static) {
    self._listeners.push(Box::new(listener));
  }

  /// Removes a listener from the client.
  pub fn remove_listener(&mut self, listener: &impl DiagnosticListener) {
    self._listeners.retain(|it| !std::ptr::eq(it.as_ref(), listener));
  }
}

mod universal {
  //! Universal implementation details for the diagnostic server.
  use super::*;

  impl DiagnosticServer {
    pub async fn start_tcp(_address: impl AsRef<str>) -> Result<DiagnosticServer, DiagnosticError> {
      todo!()
    }
  }

  impl DiagnosticClient {
    pub async fn connect_tcp(_address: impl AsRef<str>) -> Result<DiagnosticClient, DiagnosticError> {
      todo!()
    }
  }
}
