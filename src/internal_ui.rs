use amethyst::ecs::prelude::{DenseVecStorage,Component,World};
use amethyst::assets::{AssetStorage,Loader};
use amethyst::{
    core::{
        nalgebra::Vector3,
        transform::{GlobalTransform, Transform},
    },
    prelude::*,
    renderer::{SpriteRender, SpriteSheetHandle,ScreenDimensions},
};
use amethyst::{
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub fn initialize_ui(world: &mut World,ui_sheet: SpriteSheetHandle) {
    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width() as f32,dimn.height() as f32)
    };
    let font = world.read_resource::<Loader>().load(
        "SaucerBB.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    
    let names = ["Default Tower", "Fire Tower", "Ice Tower","Sacrifice Tile"].iter().map(|s| s.to_string());
    let start_height = height /  ( names.len() + 1 ) as f32;
    for (idx,name) in names.enumerate() {
        let default_trans = UiTransform::new(
            name.clone(), Anchor::BottomLeft,
            width / 10.0, start_height + idx as f32 * start_height, 0.0, 400.0, 30.0, idx as i32,
        );

        let default_ui = world.create_entity()
            .with(default_trans)
            .with(UiText::new(
                font.clone(),
                name.clone(),
                [0.2, 0.9, 1.0, 1.0],
                25.,
            )).build();
    }

    let fps_trans = UiTransform::new(
            "fps".to_owned(), Anchor::TopLeft,
            100.0, -16.0, 0.0, 200.0, 25.0, 5,
    );

    let default_ui = world.create_entity()
        .with(fps_trans)
        .with(UiText::new(
            font.clone(),
            "".to_owned(),
            [0.2, 0.9, 1.0, 1.0],
            25.,
        )).build();
}