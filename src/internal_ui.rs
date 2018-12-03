use amethyst::ecs::prelude::{
    DenseVecStorage,Component,World
};
use amethyst::{
    core::{
        cgmath::Vector3,
        transform::{GlobalTransform, Transform},
    },
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle,ScreenDimensions},
};

pub fn initialize_ui(world: &mut World,ui_sheet: SpriteSheetHandle) {
    let (_,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width(),dimn.height())
    };

    for i in (1..4) {
        let sprite = SpriteRender {
            sprite_sheet: ui_sheet.clone(),
            sprite_number: i - 1,
            flip_horizontal: false,
            flip_vertical: false,
        };

        world.create_entity()
            .with(Transform::from(Vector3::new(40.0, i as f32 * ( height / 3.0),0.0)))
            .with(sprite)
            .build();
    }
} 