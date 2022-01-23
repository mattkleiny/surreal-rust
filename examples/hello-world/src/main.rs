use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Hello, World!",
    size: (1920 / 2, 1080 / 2),
  };

  let platform = DesktopPlatform::new(configuration)
      .expect("Failed to create platform!");

  platform.run(|platform| {
    let mut commands = CommandBuffer::new();

    commands.enqueue(Command::SetViewport(Viewport { width: 1920, height: 1080 }));
    commands.enqueue(Command::ClearColor(Color::BLACK));

    platform.execute_command_queue(commands);
  });
}
