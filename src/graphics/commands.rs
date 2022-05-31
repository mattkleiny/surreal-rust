use super::*;

pub struct CommandBuffer {
  server: GraphicsServer,
  commands: Vec<Command>,
}

enum Command {
  ClearColor(Color),
  ClearDepth,
}

impl CommandBuffer {
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      server: server.clone(),
      commands: Vec::new(),
    }
  }

  pub fn clear_color_buffer(&mut self, color: Color) {
    self.commands.push(Command::ClearColor(color));
  }

  pub fn clear_depth_buffer(&mut self) {
    self.commands.push(Command::ClearDepth);
  }

  pub fn flush(&mut self) {
    while let Some(command) = self.commands.pop() {
      self.execute_command(command);
    }
  }

  fn execute_command(&mut self, command: Command) {
    match command {
      Command::ClearColor(color) => {
        self.server.clear_color_buffer(color);
      }
      Command::ClearDepth => {
        self.server.clear_depth_buffer();
      }
    }
  }
}

pub struct Renderer {
  passes: Vec<Box<dyn RenderPass>>,
  commands: CommandBuffer,
}

impl Renderer {
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      passes: Vec::new(),
      commands: CommandBuffer::new(server),
    }
  }

  pub fn add_pass(&mut self, pass: impl RenderPass + 'static) {
    self.passes.push(Box::new(pass));
    self.passes.sort_by_key(|pass| pass.order());
  }

  pub fn render(&mut self) {
    let commands = &mut self.commands;

    for pass in &mut self.passes {
      pass.begin_frame(commands);
    }

    for pass in &mut self.passes {
      pass.render(commands);
    }

    for pass in &mut self.passes {
      pass.end_frame(commands);
    }

    commands.flush();
  }
}

pub trait RenderPass {
  fn order(&self) -> usize;

  fn begin_frame(&mut self, commands: &mut CommandBuffer);
  fn render(&mut self, commands: &mut CommandBuffer);
  fn end_frame(&mut self, commands: &mut CommandBuffer);
}

struct SpritePass {}

impl RenderPass for SpritePass {
  fn order(&self) -> usize {
    0
  }

  fn begin_frame(&mut self, _commands: &mut CommandBuffer) {
    todo!()
  }

  fn render(&mut self, commands: &mut CommandBuffer) {
    commands.clear_color_buffer(Color::WHITE);
  }

  fn end_frame(&mut self, _commands: &mut CommandBuffer) {
    todo!()
  }
}
