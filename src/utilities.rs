use amethyst::renderer::{TextureMetadata,ScreenDimensions,Projection,Camera,PngFormat,TextureHandle,Texture,Sprite,SpriteSheetHandle,TextureCoordinates,SpriteSheet};
use amethyst::assets::{AssetStorage,Loader};
use amethyst::prelude::*;
use amethyst::core::nalgebra::{Vector3,Orthographic3};
use amethyst::core::transform::{Transform,GlobalTransform};

fn load_texture_from_image(world: &mut World,image_path: &str,texture_id: u64) -> TextureHandle {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
            image_path,
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &texture_storage)
}
pub fn decompile_as_sprites(world: &mut World,image_path: &str,image_size: (f32,f32), sprite_size: (f32,f32),texture_id: u64,grid:bool) -> SpriteSheetHandle {    
    let sprites_in_x = (image_size.0 / sprite_size.0).trunc();
    let sprites_in_y = (image_size.1 / sprite_size.1).trunc();

    let sprite_offset_x_in_image = 1.0 / sprites_in_x;
    let sprite_offset_y_in_image = 1.0 / sprites_in_y;

    let mut sprites = vec![];

    for y in 0..sprites_in_y as u32{
        for x in 0..sprites_in_x as u32{
            let left = x as f32 * sprite_offset_x_in_image;
            let right = (x + 1) as f32 * sprite_offset_x_in_image;

            let top = (y + 1) as f32 * sprite_offset_y_in_image;
            let bottom = y as f32 * sprite_offset_y_in_image;

            let tex_coords = TextureCoordinates{
                left,
                right,
                bottom,
                top,
            };
            
            let offsets = if grid {
                [-sprite_size.0 /2.0, -sprite_size.1 / 2.0]
            } else {
                [0.0, 0.0]
            };

            let sprite = Sprite {
                width: sprite_size.0,
                height: sprite_size.1,
                offsets: offsets,
                tex_coords,
            };

            sprites.push(sprite);
        }
    }
    let texture_handle = load_texture_from_image(world, image_path, texture_id);

    let sprite_sheet = SpriteSheet{
        texture: texture_handle,
        sprites: sprites,
    };


    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        loader.load_from_data(sprite_sheet, (), &sprite_sheet_storage)
    };
    sprite_sheet_handle
}

pub fn initialise_camera(world: &mut World) {
    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width(), dimn.height())
    };

    let v = Vector3::new(- width / 2.0,- height / 2.0, 1.0);

    world.create_entity()
        .with(Camera::from(Projection::Orthographic(Orthographic3::new(
            0.0,
            width,
            0.0,
            height,
            0.0,
            2000.0,
        ))))
        .with(Transform::from(v))
        .build();
}
