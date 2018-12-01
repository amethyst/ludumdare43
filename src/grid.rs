use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub struct Grid {
  length: f64,
  width: f64,
}

impl Grid {
  pub fn new(length: f64, width: f64) -> Grid {
    Grid {
      length,
      width
    }
  }
}

impl Component for Grid {
  type Storage = DenseVecStorage<Self>;
}

pub fn initialize_grid(world: &mut World) {
  let mut grid_transform = Transform::default();
}

