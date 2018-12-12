use amethyst::core::nalgebra::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component,NullStorage, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle, ScreenDimensions};

use rand::Rng;
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

#[derive(PartialEq,Eq)]
pub enum TileType {
  Pathable,
  Buildable(bool),
}

#[derive(Default)]
pub struct Entry;

impl Component for Entry {
  type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Exit;

impl Component for Exit {
  type Storage = NullStorage<Self>;
}

pub struct Tile {
  pub state: TileType,
}
impl Tile {
  pub const TILE_SIZE: usize = 64;
}

impl Component for Tile {
  type Storage = DenseVecStorage<Self>;
}

impl Default for Tile {
  fn default() -> Self {
    Self {
      state: TileType::Buildable(false)
    }
  }
}

pub fn initialize_grid(world: &mut World, sprite_sheet_handle: SpriteSheetHandle,size: (i32,i32)) {
    assert!(size.0 % 2 == 0 && size.1 % 2 == 0);

    let start = (-size.0 / 2 * Tile::TILE_SIZE as i32,-size.1 / 2 * Tile::TILE_SIZE as i32);
    let end = ( size.0 * Tile::TILE_SIZE as i32 + start.0, size.1 * Tile::TILE_SIZE as i32 + start.1);
    
    let tile_render = SpriteRender {
          sprite_sheet: sprite_sheet_handle,
          sprite_number: 0, 
    };
    
    let max_blocks = ( size.0 * size.1 ) as f32 / 4.0; 
    let blank_fields = (0..max_blocks as i32).map(|_| (rand::thread_rng().gen_range(0,size.0) as usize,rand::thread_rng().gen_range(0,size.1) as usize ) ).collect::<Vec<_>>();
    
    let now = Instant::now();
    for (x_id,x) in (start.0..end.0).step_by(Tile::TILE_SIZE).enumerate() {
      for (y_id,y) in (start.1..end.1).step_by(Tile::TILE_SIZE).enumerate() {
          //if let None = blank_fields.iter().find(|(dx,dy)| (*dx,*dy) == (x_id,y_id)) {
            world
              .create_entity()
              .with(tile_render.clone())
              .with(Tile::default())
              .with(Transform::from(Vector3::new(x as f32,y as f32,0.0)))
              .build();
          //}
      }
   }
    println!("Time to generate: {:?}", Instant::now().duration_since(now));
}

