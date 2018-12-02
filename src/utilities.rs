use amethyst::renderer::{TextureMetadata,ScreenDimensions,Projection,Camera,PngFormat,Texture,MaterialTextureSet,Sprite,SpriteSheetHandle,TextureCoordinates,SpriteSheet};
use amethyst::assets::{AssetStorage,Loader};
use amethyst::prelude::*;
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform};

fn load_texture_from_image(world: &mut World,image_path: &str,texture_id: u64) {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let texture_handle = loader.load(
            image_path,
            PngFormat,
            TextureMetadata::srgb(),
            (),
            &texture_storage);
    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    material_texture_set.insert(texture_id, texture_handle); 
}
pub fn decompile_as_sprites(world: &mut World,image_path: &str,image_size: (f32,f32), sprite_size: (f32,f32),texture_id: u64) -> SpriteSheetHandle {    
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

            let sprite = Sprite {
                width: sprite_size.0,
                height: sprite_size.1,
                //offsets: [sprite_size.0 / 2.0, sprite_size.1 / 2.0],
                offsets: [0.0, 0.0],
                tex_coords,
            };

            sprites.push(sprite);
        }
    }

    let sprite_sheet = SpriteSheet{
        texture_id: texture_id,
        sprites: sprites,
    };

    load_texture_from_image(world, image_path, texture_id);

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

    
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            width,
            height,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}
