//! A server for exposing diagnostic information to clients.
//!
//! This is primarily used for debugging and profiling, and is not intended to
//! be used in production.

use crate::{Channel, FromVariant, Protocol, TimeStamp, ToVariant, Variant, VariantError};

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
  Counter {
    name: String,
    value: i64,
  },
  Telemetry {
    name: &'static str,
    value: f64,
  },
}

impl ToVariant for DiagnosticEvent {
  fn to_variant(&self) -> Variant {
    todo!()
  }
}

impl FromVariant for DiagnosticEvent {
  fn from_variant(variant: Variant) -> Result<Self, VariantError> {
    todo!()
  }
}

/// A listener that can receive diagnostic events.
pub trait DiagnosticListener {
  fn on_debug_event(&mut self, event: &DiagnosticEvent);
}

/// Allows a closure to be used as a [`DiagnosticListener`].
impl<F> DiagnosticListener for F
where
  F: for<'a> Fn(&'a DiagnosticEvent),
{
  fn on_debug_event(&mut self, event: &DiagnosticEvent) {
    self(event)
  }
}

/// A potential error that can occur when interacting with diagnostics.
#[derive(Debug)]
pub enum DiagnosticError {
  FailedToStart,
  FailedToConnect,
  FailedToSend,
}

/// A [`Protocol`] for diagnostic information.
pub struct DiagnosticProtocol;

impl Protocol for DiagnosticProtocol {
  type Command = DiagnosticEvent;
  type Event = ();
}

/// A server that exposes diagnostic information to clients.
pub struct DiagnosticServer {
  channel: Box<dyn Channel<DiagnosticProtocol>>,
}

impl DiagnosticServer {
  /// Creates a new diagnostic server on the given channel.
  pub fn from_channel(channel: impl Channel<DiagnosticProtocol> + 'static) -> Self {
    Self {
      channel: Box::new(channel),
    }
  }

  /// Notifies all connected clients of a [`DiagnosticEvent`].
  pub fn notify(&mut self, event: DiagnosticEvent) -> Result<(), DiagnosticError> {
    self.channel.send(event).map_err(|_| DiagnosticError::FailedToSend)
  }
}

/// A client that can connect to a diagnostic server.
pub struct DiagnosticClient {
  _channel: Box<dyn Channel<DiagnosticProtocol>>,
  _listeners: Vec<Box<dyn DiagnosticListener>>,
}

impl DiagnosticClient {
  /// Creates a new diagnostic client on the given channel.
  pub fn from_channel(channel: impl Channel<DiagnosticProtocol> + 'static) -> Self {
    Self {
      _channel: Box::new(channel),
      _listeners: Vec::new(),
    }
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

/// Indicates a kind of telemetry.
pub trait Telemetry {
  /// The name of the telemetry.
  fn name(&self) -> &'static str;
}

/// Implements [`Telemetry`] for the given type.
#[macro_export]
macro_rules! impl_telemetry {
  ($type:ty, $name:expr) => {
    impl Telemetry for $type {
      fn name(&self) -> &'static str {
        $name
      }
    }
  };
}

pub struct FramesPerSecond(pub u32);
pub struct FrameTime(pub std::time::Duration);
pub struct FrameTimeAverage(pub std::time::Duration);
pub struct FrameTimeMinimum(pub std::time::Duration);
pub struct FrameTimeMaximum(pub std::time::Duration);

impl_telemetry!(FramesPerSecond, "frames_per_second");
impl_telemetry!(FrameTime, "frame_time");
impl_telemetry!(FrameTimeAverage, "frame_time_average");
impl_telemetry!(FrameTimeMinimum, "frame_time_minimum");
impl_telemetry!(FrameTimeMaximum, "frame_time_maximum");

#[cfg(test)]
mod tests {
  use crate::{DiagnosticClient, DiagnosticServer, TcpClient, TcpServer};

  #[test]
  fn test_server_client_interaction() {
    let mut server = DiagnosticServer::from_channel(TcpServer::new("127.0.0.1:7337").unwrap());
    let mut client = DiagnosticClient::from_channel(TcpClient::connect("127.0.0.1:7337").unwrap());

    server
      .notify(super::DiagnosticEvent::Log {
        level: super::super::LogLevel::Info,
        message: "Hello, world!".to_string(),
      })
      .unwrap();
  }
}
