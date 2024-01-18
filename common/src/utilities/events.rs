use std::sync::mpsc;

/// A simple event bus for an event type `E`
pub struct EventBus<E> {
  sender: mpsc::Sender<E>,
  receiver: mpsc::Receiver<E>,
}

impl<E> EventBus<E> {
  /// Creates a new event bus.
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::channel();

    Self { sender, receiver }
  }

  /// Sends an event to all listeners.
  pub fn send(&self, event: E) {
    self.sender.send(event).unwrap();
  }

  /// Attempts to receive an event.
  pub fn receive(&self) -> Option<E> {
    self.receiver.try_recv().ok()
  }

  /// Returns an iterator over all events.
  pub fn iter(&self) -> impl Iterator<Item = E> + '_ {
    self.receiver.try_iter()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_send_receive() {
    let event_bus: EventBus<u32> = EventBus::new();

    event_bus.send(1);
    event_bus.send(2);
    event_bus.send(3);

    assert_eq!(event_bus.receive(), Some(1));
    assert_eq!(event_bus.receive(), Some(2));
    assert_eq!(event_bus.receive(), Some(3));
    assert_eq!(event_bus.receive(), None);
  }

  #[test]
  fn test_iter() {
    let event_bus: EventBus<u32> = EventBus::new();

    event_bus.send(1);
    event_bus.send(2);
    event_bus.send(3);

    let events: Vec<u32> = event_bus.iter().collect();

    assert_eq!(events, vec![1, 2, 3]);
  }
}
