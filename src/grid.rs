use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle, ScreenDimensions};


use std::time::Instant;

impl Component for Tile {
  type Storage = DenseVecStorage<Self>;
}
#[derive(PartialEq,Eq)]
pub enum TileState {
  Hovered,
  Selected,
  None,
}

pub struct Tile {
  pub state: TileState,
}
impl Tile {
  pub const TILE_SIZE: usize = 64;
}

impl Default for Tile {
  fn default() -> Self {
    Self {
      state: TileState::None
    }
  }
}

pub fn initialize_grid(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width() as usize, dimn.height() as usize)
    };

    let tile_render = SpriteRender {
          sprite_sheet: sprite_sheet_handle,
          sprite_number: 0, 
          flip_horizontal: false,
          flip_vertical: false,
    };
    let now = Instant::now();
    for x in (0..width).step_by(Tile::TILE_SIZE) {
      for y in (0..height).step_by(Tile::TILE_SIZE) {
        world
          .create_entity()
          .with(tile_render.clone())
          .with(Tile::default())
          .with(Transform::from(Vector3::new(x as f32,y as f32,0.0)))
          .build();
      }
   }
    println!("Time to generate: {:?}", Instant::now().duration_since(now));
}

