use surreal::graphics::Texel;

use super::GraphicsServer;
use super::TextureId;

pub struct Texture {
  server: GraphicsServer,
  texture_id: TextureId,
}

impl Texture {
  pub fn new(server: &GraphicsServer) -> surreal::Result<Self> {
    Ok(Self {
      server: server.clone(),
      texture_id: server.texture_create()?,
    })
  }

  pub fn id(&self) -> TextureId {
    self.texture_id
  }

  pub fn write_pixels<P: Texel>(&mut self, pixels: &[P]) -> surreal::Result<()> {
    let pixels = unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const u8, pixels.len() * std::mem::size_of::<P>()) };

    self.server.texture_write(self.texture_id, pixels)
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    self.server.texture_delete(self.texture_id).expect("Failed to delete texture");
  }
}
