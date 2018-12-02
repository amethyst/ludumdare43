use amethyst::prelude::*;
use amethyst::ecs::prelude::{Join,Entities,WriteStorage,ReadStorage};
use amethyst::renderer::{Hidden,VirtualKeyCode};
use amethyst::input;
use amethyst::core::transform::Transform;
use amethyst::ui::UiTransform;
use utilities::{
    decompile_as_sprites,
    initialise_camera,
    Backpack,
};
use custom_game_data::CustomGameData;
use grid;

pub const GRID_HEIGHT: f32 = 600.0;
pub const GRID_WIDTH: f32 = 600.0;

pub struct TowerDefense;

impl<'a, 'b> State<CustomGameData<'a,'b>,StateEvent> for TowerDefense {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let mut world = data.world;
        let sprite_sheet_handle = grid::load_sprite_sheet(world);
        initialise_camera(&mut world);
        grid::initialize_grid(&mut world, sprite_sheet_handle.clone());
    }
    fn handle_event(&mut self, _data: StateData<CustomGameData>, event: StateEvent) ->  Trans<CustomGameData<'a,'b>,StateEvent> {
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a,'b>,StateEvent>{
        data.data.update(&data.world,true);
        Trans::None
    }
    fn on_pause(&mut self, data: StateData<CustomGameData>) {
    }
    fn on_resume(&mut self, data: StateData<CustomGameData>) {
    }
}
