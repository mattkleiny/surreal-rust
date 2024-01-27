//! An IPC server for exposing diagnostic information to clients.
//!
//! This is primarily used for debugging and profiling, and is not intended to
//! be used in production.

/// An event that can be sent to a diagnostic listener.
pub enum DiagnosticEvent {
  Log {
    level: super::LogLevel,
    message: String,
  },
  Profile {
    name: String,
    start: std::time::Instant,
    end: std::time::Instant,
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
pub enum DiagnosticServerError {}

/// A server that exposes diagnostic information to clients.
pub struct DiagnosticServer {}

impl DiagnosticServer {
  pub async fn start(name: &str) -> Result<DiagnosticServer, DiagnosticServerError> {
    todo!()
  }
}

/// A potential error that can occur when connecting to a diagnostic server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticClientError {}

/// A client that can connect to a diagnostic server.
pub struct DiagnosticClient {
  _listeners: Vec<Box<dyn DiagnosticListener>>,
}

impl DiagnosticClient {
  /// Connects to a diagnostic server at the given address.
  pub async fn connect(_address: impl AsRef<str>) -> Result<DiagnosticClient, DiagnosticClientError> {
    Ok(Self { _listeners: Vec::new() })
  }

  /// Adds a listener to the client.
  pub fn add_listener(&mut self, listener: impl DiagnosticListener + 'static) {
    self._listeners.push(Box::new(listener));
  }

  /// Removes a listener from the client.
  pub fn remove_listener(&mut self, listener: &impl DiagnosticListener) {
    self._listeners.retain(|it| !std::ptr::eq(it.as_ref(), listener));
  }
}

#[cfg(target_os = "windows")]
mod windows {
  //! Windows-specific implementation details for the diagnostic server.

  use super::*;

  impl DiagnosticServer {
    pub async fn start_named_pipes(name: &'static str) -> Result<DiagnosticServer, DiagnosticServerError> {
      todo!()
    }
  }
}

#[cfg(target_os = "unix")]
mod unix {
  //! Unix-specific implementation details for the diagnostic server.

  use super::*;

  impl DiagnosticServer {}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_server_client_interaction() {
    use crate::BlockableFuture;

    // let _server = DiagnosticServer::start(1234).block();
    // let _client = DiagnosticClient::connect("localhost:1234").block();
  }
}
