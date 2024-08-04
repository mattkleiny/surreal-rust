use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// Represents an error while communicating via TCP.
#[derive(Debug)]
pub enum TcpError {
  FailedToConnect,
  FailedToAccept,
  FailedToSend,
  FailedToReceive,
}

/// A server for TCP communication.
pub struct TcpServer {
  listener: TcpListener,
}

impl TcpServer {
  /// Create a new TCP server that listens on the given address.
  pub fn new(address: impl ToSocketAddrs) -> Result<Self, TcpError> {
    let listener = TcpListener::bind(address).map_err(|_| TcpError::FailedToConnect)?;

    Ok(Self { listener })
  }

  /// Accept a new connection from a client.
  pub fn accept(&self) -> Result<TcpClient, TcpError> {
    let (stream, _) = self.listener.accept().map_err(|_| TcpError::FailedToAccept)?;

    Ok(TcpClient { _stream: stream })
  }

  /// Send data to all clients.
  pub fn send(&self, _data: &[u8]) -> Result<(), TcpError> {
    todo!()
  }

  /// Receive data from all clients.
  pub fn receive(&self) -> Result<Vec<u8>, TcpError> {
    todo!()
  }
}

/// A client for TCP communication.
pub struct TcpClient {
  _stream: TcpStream,
}

impl TcpClient {
  /// Connect to a server at the given address.
  pub fn connect(address: impl ToSocketAddrs) -> Result<Self, TcpError> {
    let stream = TcpStream::connect(address).map_err(|_| TcpError::FailedToConnect)?;

    Ok(Self { _stream: stream })
  }

  /// Send data to the server.
  pub fn send(&self, _data: &[u8]) -> Result<(), TcpError> {
    todo!()
  }

  /// Receive data from the server.
  pub fn receive(&self) -> Result<Vec<u8>, TcpError> {
    todo!()
  }
}
