use amethyst::prelude::*;
use amethyst::ecs::prelude::{Join,Entities,WriteStorage,ReadStorage};
use amethyst::renderer::{Hidden,VirtualKeyCode};
use amethyst::input;
use amethyst::core::transform::Transform;
use utilities::{
    decompile_as_sprites,
    initialise_camera,
};
use enemy::{initialise_enemy,Enemy};
use towers::{initialise_tower,Tower};
use custom_game_data::CustomGameData;
use bullet::Bullet;
use tracker::GameTracker;
use backpack::Backpack;
use collider::CircleCollider;
use grid::{initialize_grid,Tile};
use internal_ui::initialize_ui;
pub const GAME_HEIGHT: f64 = 600.0;
pub const GAME_WIDTH: f64 = 600.0;

pub struct TowerDefense;

impl<'a, 'b> State<CustomGameData<'a,'b>,StateEvent> for TowerDefense {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let mut world = data.world;
        let sheet_handle = decompile_as_sprites(&mut world, "towers.png", (128.0,128.0), (64.0,64.0), 0,false);
        let bullet_sheet = decompile_as_sprites(&mut world, "bullets.png", (64.0,64.0), (32.0,32.0), 1,false);
        let grid_sheet = decompile_as_sprites(&mut world,"tiles.png", (192.0, 64.0), (Tile::TILE_SIZE as f32, Tile::TILE_SIZE as f32), 2, true);
        let button_sheet = decompile_as_sprites(&mut world, "buttons.png", (256.0,64.0), (64.0,64.0), 3,false);

        world.register::<CircleCollider>();

        initialise_camera(&mut world);
        initialize_grid(&mut world, grid_sheet.clone(),3);
        initialize_ui(&mut world,button_sheet);
        
        world.add_resource(GameTracker::default());
        world.add_resource(Backpack::new(sheet_handle,bullet_sheet,grid_sheet));
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
