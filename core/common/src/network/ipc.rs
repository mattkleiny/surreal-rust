//! Lightweight abstractions for Inter-process communication.

use crate::{Deserialize, Serialize, TcpServer};

/// An error that can occur during IPC communication.
#[derive(Debug)]
pub enum IpcError {
  SendError,
  ReceiveError,
}

/// A protocol for IPC communication.
///
/// This trait defines the types of commands and events that can be sent between
/// processes. It's expected that one of the parties in the protocol is a
/// 'server' that listens for commands and sends events, while the other party
/// is a 'client' that sends commands and listens for events.
pub trait IpcProtocol {
  type Command;
  type Event;
}

/// A channel for IPC communication.
pub trait IpcChannel<P: IpcProtocol> {
  /// Send a command to the other party.
  fn send(&self, command: P::Command) -> Result<(), IpcError>;

  /// Receive an event from the other party.
  fn receive(&self) -> Result<Option<P::Event>, IpcError>;
}

/// Allow socket-based IPC communication.
impl<P: IpcProtocol> IpcChannel<P> for TcpServer
where
  P::Command: Serialize + Deserialize,
  P::Event: Serialize + Deserialize,
{
  fn send(&self, command: P::Command) -> Result<(), IpcError> {
    let bytes = command.to_binary_bytes().map_err(|_| IpcError::SendError)?;

    TcpServer::send(&self, &bytes).map_err(|_| IpcError::SendError)
  }

  fn receive(&self) -> Result<Option<P::Event>, IpcError> {
    let bytes = TcpServer::receive(&self).map_err(|_| IpcError::ReceiveError)?;

    if bytes.is_empty() {
      return Ok(None);
    }

    let event = P::Event::from_binary_bytes(&bytes).map_err(|_| IpcError::ReceiveError)?;

    Ok(Some(event))
  }
}

/// Allow socket-based IPC communication.
impl<P: IpcProtocol> IpcChannel<P> for super::TcpClient
where
  P::Command: Serialize + Deserialize,
  P::Event: Serialize + Deserialize,
{
  fn send(&self, _command: P::Command) -> Result<(), IpcError> {
    unimplemented!()
  }

  fn receive(&self) -> Result<Option<P::Event>, IpcError> {
    unimplemented!()
  }
}
