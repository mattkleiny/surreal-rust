//! Lightweight abstractions for channel-based communication.

use crate::{Deserialize, Serialize, TcpServer};

/// An error that can occur during channel I/O.
#[derive(Debug)]
pub enum ChannelError {
  FailedToSend,
  FailedToReceive,
}

/// A protocol for channel-based communication.
pub trait Protocol {
  type Command;
  type Event;
}

/// Represents a channel on some protocol [`P`].
pub trait Channel<P: Protocol> {
  /// Send a [`P::Command`] to the other party.
  fn send(&self, command: P::Command) -> Result<(), ChannelError>;

  /// Receive an [`P::Event`] from the other party.
  fn receive(&self) -> Result<Option<P::Event>, ChannelError>;

  /// Receive all available [`P::Event`]s from the other party.
  fn receive_all(&self) -> Result<Vec<P::Event>, ChannelError> {
    let mut events = Vec::new();

    while let Some(event) = self.receive()? {
      events.push(event);
    }

    Ok(events)
  }
}

/// [`Channel`] implementation for [`TcpServer`].
impl<P: Protocol> Channel<P> for TcpServer
where
  P::Command: Serialize,
  P::Event: Deserialize,
{
  fn send(&self, command: P::Command) -> Result<(), ChannelError> {
    todo!()
  }

  fn receive(&self) -> Result<Option<P::Event>, ChannelError> {
    todo!()
  }
}

/// [`Channel`] implementation for [`TcpClient`].
impl<P: Protocol> Channel<P> for super::TcpClient
where
  P::Command: Serialize,
  P::Event: Deserialize,
{
  fn send(&self, _command: P::Command) -> Result<(), ChannelError> {
    unimplemented!()
  }

  fn receive(&self) -> Result<Option<P::Event>, ChannelError> {
    unimplemented!()
  }
}
