extern crate amethyst;
extern crate rand;

use amethyst::{
    LoggerConfig,StdoutLog,
    core::{
        transform::TransformBundle, 
    },
    prelude::*,
    renderer::{DisplayConfig,DrawFlat2D,Pipeline, RenderBundle, Stage,ColorMask,ALPHA},
    input::InputBundle,
    ui::{UiBundle,DrawUi},
    utils::fps_counter::{FPSCounter, FPSCounterBundle},
};
mod custom_game_data;
mod game;
mod utilities;
mod systems;
mod towers;
mod enemy;
mod bullet;
mod tracker;
mod backpack;
mod collider;
mod grid;
mod internal_ui;

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
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                None,
            ))
            .with_pass(DrawUi::new()),
    );

    let game_data = CustomGameDataBuilder::default()
        .with_bundle(TransformBundle::new(), DispatchData::Core)?
        .with_bundle(UiBundle::<String,String>::new(),DispatchData::Core)?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor().with_sprite_visibility_sorting(&["transform_system"]), DispatchData::Core)?
        .with_bundle(InputBundle::<String, String>::new(),DispatchData::Core)?
        .with_bundle(FPSCounterBundle::default(), DispatchData::Gameplay)?
        .with_bundle(systems::gameplay::TowerDefenseBundle,DispatchData::Gameplay)?;

    Application::build(assets_dir, TowerDefense {fps_display: None})?
        .build(game_data)?
        .run();
    Ok(())
}