use anyhow::anyhow;

/// Represents a sprite that can be packed by the sprite sheet packer.
#[derive(Clone, Eq, PartialEq)]
pub struct Sprite {
  pub id: usize,
  pub size: (u32, u32),
  pub pixels: Vec<u32>,
}

/// Encodes an anchor position used in positioning [`Sprite`]s.
#[derive(Clone, Debug)]
pub struct SpriteAnchor<'a> {
  pub position: (u32, u32),
  pub sprite: &'a Sprite,
}

/// A sprite sheet as output from the packer.
///
/// The sheet describes which sprites (identified by their ID) are present in
/// the sheet, and where they are located. A separate step should be used to
/// build the resultant texture/image data.
#[derive(Clone, Debug)]
pub struct SpriteSheet<'a> {
  pub width: u32,
  pub height: u32,
  pub anchors: Vec<SpriteAnchor<'a>>,
}

/// Packs the given sprites as a uniform set of images into a [`SpriteSheet`].
///
/// This method requires that all sprites have the same size and will form a
/// rectangular grid of the least highest power of 2 necessary to fit all
/// sprites.
pub fn pack_uniform_grid(sprites: &mut [Sprite]) -> anyhow::Result<SpriteSheet> {
  let _width = sprites.iter().map(|sprite| sprite.size.0).max().unwrap();
  let _height = sprites.iter().map(|sprite| sprite.size.1).max().unwrap();

  todo!()
}

/// Packs the given set of `Sprite`s into a [`SpriteSheet`].
pub fn pack_sprite_sheet(sprites: &mut [Sprite]) -> anyhow::Result<SpriteSheet> {
  let mut positions = Vec::new();
  let mut anchors = Vec::new();

  sprites.sort_by(compare_size);
  sprites.reverse();

  positions.push((0, 0));

  for sprite in sprites {
    // add sprite to this free position
    let next_pos = *positions.first().ok_or(anyhow!("No free anchor positions!"))?;

    anchors.push(SpriteAnchor {
      position: next_pos,
      sprite,
    });

    // find new anchors
    let mut new_right = (next_pos.0 + sprite.size.0, next_pos.1);
    let mut new_bottom = (next_pos.0, next_pos.1 + sprite.size.1);

    // still finding new anchors
    for i in 1..(positions.len() - 1) {
      // If we removed an anchor after the first round,
      // we might be out of bounds at this point
      if i > 1 && i >= positions.len() {
        break;
      }

      if positions[i].0 >= positions[0].0 && positions[i].0 <= new_right.0 {
        new_right.1 = std::cmp::min(new_right.1, positions[i].1);
        positions.remove(i);
        continue;
      }

      if positions[i].1 >= positions[0].1 && positions[i].1 <= new_bottom.1 {
        new_bottom.0 = std::cmp::min(new_bottom.0, positions[i].0);
        positions.remove(i);
        continue;
      }
    }

    // remove first, push new anchors
    positions.remove(0);

    if !positions.contains(&new_right) {
      positions.push(new_right);
    }

    if !positions.contains(&new_bottom) {
      positions.push(new_bottom);
    }

    positions.sort_by(compare_pos);
  }

  let width = positions
    .iter()
    .max_by(|a, b| a.0.cmp(&b.0))
    .ok_or(anyhow!("Invalid: No free anchors"))?
    .0;

  let height = positions
    .iter()
    .max_by(|a, b| a.1.cmp(&b.1))
    .ok_or(anyhow!("Invalid: No free anchors"))?
    .1;

  // Finally sort the anchors so that they are in the same order as the input
  // sprites
  anchors.sort_by_key(|s| s.sprite.id);

  Ok(SpriteSheet { width, height, anchors })
}

/// Compares the position of two sprites.
fn compare_pos(a: &(u32, u32), b: &(u32, u32)) -> std::cmp::Ordering {
  // cast to 64 bit; we'd overflow quickly for larger sprite sheets otherwise
  let a = (a.0 as u64, a.1 as u64);
  let b = (b.0 as u64, b.1 as u64);

  (a.0.pow(4) + a.1.pow(4)).cmp(&(b.0.pow(4) + b.1.pow(4)))
}

/// Compares the size of two sprites.
fn compare_size(a: &Sprite, b: &Sprite) -> std::cmp::Ordering {
  (a.size.0 * a.size.1).cmp(&(b.size.0 * b.size.1))
}

/// Computes the next highest power of 2 of the given number.
fn _next_highest_power_of_2(n: u32) -> u32 {
  let n = (n - 1) as f32;
  let log = n.log2();

  1u32 << ((log as u32) + 1)
}

impl std::fmt::Debug for Sprite {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Sprite").field("id", &self.id).field("size", &self.size).finish()
  }
}

#[cfg(test)]
mod tests {
  use image::Rgba;

  use super::*;

  #[test]
  fn it_should_pack_a_simple_set_of_sprites() {
    let mut sprites = (0..16)
      .map(|i| Sprite {
        id: i,
        size: (20, 20),
        pixels: vec![0xFF00FFFF; 16 * 16],
      })
      .collect::<Vec<_>>();

    let result = pack_sprite_sheet(&mut sprites).unwrap();

    assert_eq!(result.width, 20 * 4);
    assert_eq!(result.height, 20 * 4);
  }

  #[test]
  fn it_should_pack_a_simple_sprite_sheet() {
    let bytes = std::fs::read("../../assets/sprites/tiles_desert.png").unwrap();
    let image = image::load_from_memory(&bytes).unwrap().to_rgba8();

    let mut id = 0;
    let mut sprites = Vec::new();

    // split image into 16x16 sections, as well
    for sprite_y in 0..image.height() / 16 {
      for sprite_x in 0..image.width() / 16 {
        let mut sprite_pixels = Vec::with_capacity(16 * 16);

        // pack 32 bit pixel values from raw image data
        for pixel_y in 0..16 {
          for pixel_x in 0..16 {
            let Rgba([r, g, b, a]) = image.get_pixel(sprite_x * 16 + pixel_x, sprite_y * 16 + pixel_y);

            sprite_pixels.push((*r as u32) << 24 | (*g as u32) << 16 | (*b as u32) << 8 | (*a as u32));
          }
        }

        sprites.push(Sprite {
          id,
          size: (image.width(), image.height()),
          pixels: sprite_pixels,
        });

        id += 1;
      }
    }

    assert!(pack_sprite_sheet(&mut sprites).is_ok());
  }
}
