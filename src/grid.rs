use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component,NullStorage, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle, ScreenDimensions};


use std::time::Instant;

#[derive(Default)]
pub struct Hovered;

impl Component for Hovered {
  type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Selected;

impl Component for Selected {
  type Storage = NullStorage<Self>;
}

impl Component for Tile {
  type Storage = DenseVecStorage<Self>;
}
#[derive(PartialEq,Eq)]
pub enum TileType {
  Pathable,
  Buildable(bool),
}

pub struct Tile {
  pub state: TileType,
}
impl Tile {
  pub const TILE_SIZE: usize = 64;
}

impl Default for Tile {
  fn default() -> Self {
    Self {
      state: TileType::Buildable(false)
    }
  }
}

pub fn initialize_grid(world: &mut World, sprite_sheet_handle: SpriteSheetHandle,size: u16) {
    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width(),dimn.height())
    };
    println!("{},{}",width,height);
    let start = (width / 2.0 - size as f32 * Tile::TILE_SIZE as f32) as usize;
    let end = ( size * 2 ) as usize * Tile::TILE_SIZE + start;

    let tile_render = SpriteRender {
          sprite_sheet: sprite_sheet_handle,
          sprite_number: 0, 
          flip_horizontal: false,
          flip_vertical: false,
    };
    let now = Instant::now();
    for x in (start..end).step_by(Tile::TILE_SIZE) {
      for y in (start..end).step_by(Tile::TILE_SIZE) {
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

