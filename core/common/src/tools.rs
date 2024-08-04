//! Tools framework for the engine

use std::sync::Mutex;

use crate::{EventBus, IpcChannel};

/// The protocol for communication between the engine and tools.
pub struct ToolProtocol;

impl crate::IpcProtocol for ToolProtocol {
  type Command = ToolCommand;
  type Event = ToolEvent;
}

/// A command that can be received from a tool.
pub enum ToolCommand {}

/// An event that can be sent to a tool.
pub enum ToolEvent {}

/// A server for tools to communicate with the engine.
///
/// This server listens for commands from tools and sends events to them.
pub struct ToolServer {
  server: Box<dyn IpcChannel<ToolProtocol>>,
  commands: Mutex<Vec<ToolCommand>>,
}

/// A client for the engine to communicate with tools.
///
/// This client sends commands to tools and listens for events from them.
pub struct ToolClient {
  client: Box<dyn IpcChannel<ToolProtocol>>,
  events: EventBus<ToolEvent>,
}
