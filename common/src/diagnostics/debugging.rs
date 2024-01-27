pub enum DebugEvent {}

pub trait DebugListener {
  fn on_debug_event(&mut self, event: &DebugEvent);
}

impl<F: FnMut(&DebugEvent)> DebugListener for F {
  fn on_debug_event(&mut self, event: &DebugEvent) {
    self(event)
  }
}

pub enum DebugServerError {}

pub struct DebugServer {}

impl DebugServer {
  pub async fn start(_port: usize) -> Result<DebugServer, DebugServerError> {
    todo!()
  }
}

pub enum DebugClientError {}

pub struct DebugClient {
  _listeners: Vec<Box<dyn DebugListener>>,
}

impl DebugClient {
  pub async fn connect(_address: impl AsRef<str>) -> Result<DebugClient, DebugClientError> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_server_client_interaction() {
    use crate::BlockableFuture;

    let _server = DebugServer::start(1234).block();
    let _client = DebugClient::connect("localhost:1234").block();
  }
}
