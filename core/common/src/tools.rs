//! Tools framework for the engine

pub trait ToolProtocol {
  type Command;
  type Event;
}

pub struct ToolServer<P: ToolProtocol> {
  commands: Vec<P::Command>,
  events: Vec<P::Event>,
}

pub struct ToolClient<P: ToolProtocol> {
  commands: Vec<P::Command>,
  events: Vec<P::Event>,
}
