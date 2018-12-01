extern crate amethyst;
extern crate rand;

use amethyst::{
    LoggerConfig,StdoutLog,
    core::{
        transform::TransformBundle, 
    },
    prelude::*,
    renderer::{DisplayConfig,DrawSprite, Pipeline, RenderBundle, Stage,ColorMask,ALPHA},
    input::InputBundle,
    ui::{UiBundle,DrawUi},
};
mod custom_game_data;
mod game;
mod utilities;
mod systems;
mod towers;
mod enemy;
mod bullet;
mod tracker;

use game::TowerDefense;
use custom_game_data::{CustomGameDataBuilder,DispatchData};



fn main() -> amethyst::Result<()> {
    let mut logger_config = LoggerConfig::default();
    logger_config.stdout = StdoutLog::Off;
    amethyst::start_logger(logger_config);

    let resources_dir = format!("{}/resources/", env!("CARGO_MANIFEST_DIR"));
    let assets_dir = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));

    let config = DisplayConfig::load(resources_dir + "display.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                None,
            ))
            .with_pass(DrawUi::new()),
    );

    let input_bundle = InputBundle::<String, String>::new();

    let game_data = CustomGameDataBuilder::default()
        .with_bundle(TransformBundle::new(), DispatchData::Core)?
        .with_bundle(UiBundle::<String,String>::new(),DispatchData::Core)?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor().with_sprite_visibility_sorting(&["transform_system"]), DispatchData::Core)?
        .with_bundle(input_bundle,DispatchData::Core)?
        .with_bundle(systems::gameplay::TowerDefenseBundle,DispatchData::Gameplay)?;

    Application::build(assets_dir, TowerDefense)?
        .build(game_data)?
        .run();
    Ok(())
}