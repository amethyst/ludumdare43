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
use enemy::{initialise_enemy,Enemy};
use towers::{initialise_tower,Tower};
use custom_game_data::CustomGameData;
use bullet::Bullet;
use tracker::GameTracker;

pub struct TowerDefense;

impl<'a, 'b> State<CustomGameData<'a,'b>,StateEvent> for TowerDefense {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let mut world = data.world;
        
        world.add_resource(GameTracker::default());

        initialise_camera(&mut world);
        let sheet_handle = decompile_as_sprites(&mut world, "towers.png", (128.0,128.0), (64.0,64.0), 0);
        initialise_tower(&mut world, sheet_handle.clone());
        initialise_enemy(&mut world, sheet_handle);
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
