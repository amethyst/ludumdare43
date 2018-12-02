use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::{
  core::transform::TransformBundle,
  input::InputBundle,
  prelude::*,
  renderer::{
    ColorMask, DisplayConfig, DrawSprite, MaterialTextureSet, Pipeline, PngFormat, RenderBundle,
    SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Stage, Texture, ScreenDimensions,
    TextureMetadata, ALPHA,
  },
  ui::{DrawUi, UiBundle},
  LoggerConfig, StdoutLog,
};

use std::sync::{Arc, RwLock};
use game;

pub const tile_size: f64 = 10.0;

impl Component for Tile {
  type Storage = DenseVecStorage<Self>;
}

pub struct Tile {
  pub x: f64,
  pub y: f64,
}

impl Tile {
  pub fn new(x: f64, y: f64) -> Tile {
    Tile {x, y }
  }
}

pub fn initialize_grid(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width(), dimn.height())
    };
    for i in 0..game::GRID_HEIGHT as usize {
      for j in 0..game::GRID_WIDTH as usize {
        let mut tile_transform = Transform::default();
        let x = i * 60;
        let y = j * 60;
        let sprite_id = 0;
        tile_transform.translation = Vector3::new(x as f32, y as f32, 0.0);
        let tile_render = SpriteRender {
          sprite_sheet: sprite_sheet_handle.clone(),
          sprite_number: sprite_id, // paddle is the first sprite in the sprite_sheet
          flip_horizontal: false,
          flip_vertical: false,
        };
        world
          .create_entity()
          .with(tile_render)
          .with(Tile{
            x: i as f64,
            y: j as f64,
          })
          .with(tile_transform)
          .build();
      }
    }
}

pub fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
  // Load the sprite sheet necessary to render the graphics.
  // The texture is the pixel data
  // `texture_handle` is a cloneable reference to the texture
  let texture_handle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
      "texture/white_tile.png",
      PngFormat,
      TextureMetadata::srgb_scale(),
      (),
      &texture_storage,
    )
  };
  let texture_id = 0;
  let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
  material_texture_set.insert(texture_id, texture_handle);

  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
    "texture/white_tile.ron", // Here we load the associated ron file
    SpriteSheetFormat,
    texture_id, // We pass it the handle of the texture we want it to use
    (),
    &sprite_sheet_store,
  )
}
