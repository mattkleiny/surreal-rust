use crate::ProjectDetails;

/// Central event bus for the editor itself.
#[derive(Clone)]
pub struct EventBus {
  sender: crossbeam_channel::Sender<EditorEvent>,
  receiver: crossbeam_channel::Receiver<EditorEvent>,
}

/// Events that occur in the editor.
#[derive(Debug)]
pub enum EditorEvent {
  AssetsRefreshed,
  ProjectOpened(ProjectDetails),
  ProjectClosed,
}

impl Default for EventBus {
  fn default() -> Self {
    let (sender, receiver) = crossbeam_channel::unbounded();

    Self { sender, receiver }
  }
}

impl EventBus {
  /// Sends a [`EditorEvent`] on the bus.
  pub fn send(&self, event: EditorEvent) -> surreal::Result<()> {
    surreal::diagnostics::trace!("Sending event: {event:?}");

    Ok(self.sender.send(event)?)
  }

  /// Receives a [`EditorEvent`] on the bus; blocks until a message is received.
  pub fn receive(&self) -> surreal::Result<EditorEvent> {
    Ok(self.receiver.recv()?)
  }

  /// Tries to receive a [`EditorEvent`] on the bus; returns if no message is waiting.
  pub fn try_receive(&self) -> Option<EditorEvent> {
    match self.receiver.try_recv() {
      Ok(value) => Some(value),
      Err(_) => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn event_bus_should_send_messages() {
    let event_bus = EventBus::default();

    event_bus.send(EditorEvent::ProjectClosed).unwrap();
    let event = event_bus.receive().unwrap();

    println!("{event:?}");
  }
}
