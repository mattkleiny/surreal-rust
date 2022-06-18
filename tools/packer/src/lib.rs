/// Represents a possible error whilst packing a sprite sheet.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SpritePackError {
  Unknown,
}

/// Represents a sprite that can be packed by the sprite sheet packer.
/// TODO: use a trait instead?
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sprite<'a> {
  pub id: usize,
  pub size: (u32, u32),
  pub pixels: &'a [u32],
}

/// Encodes an anchor position used in positioning sprites.
#[derive(Debug, Clone, Copy)]
pub struct SpriteAnchor {
  pub id: usize,
  pub position: (u32, u32),
  pub size: (u32, u32),
}

impl SpriteAnchor {
  pub fn new(id: usize, position: (u32, u32), size: (u32, u32)) -> Self {
    SpriteAnchor { id, position, size }
  }
}

/// A sprite sheet as output from the packer.
///
/// The sheet describes which sprites (identified by their ID) are present in the sheet, and where they are located.
/// A separate step should be used to build the resultant texture/image data.
pub struct SpriteSheet {
  pub width: u32,
  pub height: u32,
  pub anchors: Vec<SpriteAnchor>,
}

/// Packs the given set of `Sprite`s into a `SpriteSheet`.
pub fn pack_spritesheet(sprites: &[Sprite]) -> Result<SpriteSheet, SpritePackError> {
  let mut sprites: Vec<_> = sprites.iter().cloned().collect();

  let mut free = Vec::new();
  let mut anchors = Vec::new();

  sprites.sort_by(compare_size);
  sprites.reverse();

  free.push((0, 0));

  for sprite in sprites {
    // add sprite to this free position
    let next_free = *free.first().expect("No free anchor positions!");

    anchors.push(SpriteAnchor {
      id: sprite.id,
      position: next_free,
      size: sprite.size,
    });

    // find new anchors
    let mut new_right = (next_free.0 + sprite.size.0, next_free.1);
    let mut new_bottom = (next_free.0, next_free.1 + sprite.size.1);

    // still finding new anchors
    for i in 1..(free.len() - 1) {
      // If we removed an anchor after the first round,
      // we might be out of bounds at this point
      if i > 1 && i >= free.len() {
        break;
      }

      if free[i].0 >= free[0].0 && free[i].0 <= new_right.0 {
        new_right.1 = std::cmp::min(new_right.1, free[i].1);
        free.remove(i);
        continue;
      }

      if free[i].1 >= free[0].1 && free[i].1 <= new_bottom.1 {
        new_bottom.0 = std::cmp::min(new_bottom.0, free[i].0);
        free.remove(i);
        continue;
      }
    }

    // remove first, push new anchors
    free.remove(0);

    if !free.contains(&new_right) {
      free.push(new_right);
    }

    if !free.contains(&new_bottom) {
      free.push(new_bottom);
    }

    free.sort_by(compare_pos);
  }

  let width = free
    .iter()
    .max_by(|a, b| a.0.cmp(&b.0))
    .expect("Invalid: No free anchors")
    .0;

  let height = free
    .iter()
    .max_by(|a, b| a.1.cmp(&b.1))
    .expect("Invalid: No free anchors")
    .1;

  // Finally sort the anchors so that they are in the same order as the input sprites
  anchors.sort_by_key(|s| s.id);

  Ok(SpriteSheet { width, height, anchors })
}

/// Compares the position of two sprites.
fn compare_pos(a: &(u32, u32), b: &(u32, u32)) -> std::cmp::Ordering {
  (a.0.pow(4) + a.1.pow(4)).cmp(&(b.0.pow(4) + b.1.pow(4)))
}

/// Compares the size of two sprites.
fn compare_size(a: &Sprite, b: &Sprite) -> std::cmp::Ordering {
  (a.size.0 * a.size.1).cmp(&(b.size.0 * b.size.1))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_pack_a_simple_set_of_sprites() {
    let sprites = (0..16)
      .map(|i| Sprite {
        id: i,
        size: (20, 20),
        pixels: &[0xFF00FF; 16 * 16],
      })
      .collect::<Vec<_>>();

    let result = pack_spritesheet(&sprites).unwrap();

    assert_eq!(result.width, 20 * 4);
    assert_eq!(result.height, 20 * 4);
  }
}
