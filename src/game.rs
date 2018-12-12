use amethyst::prelude::*;
use amethyst::renderer::{Hidden,VirtualKeyCode};
use amethyst::input;
use amethyst::core::transform::Transform;
use amethyst::core::timing::Time;
use amethyst::utils::fps_counter::FPSCounter;
use amethyst::ecs::prelude::Entity;
use utilities::{
    decompile_as_sprites,
    initialise_camera,
};
use amethyst::{
    ui::{UiText, UiTransform,UiFinder},
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

pub struct TowerDefense {
    pub fps_display: Option<Entity>,
}

impl<'a, 'b> State<CustomGameData<'a,'b>,StateEvent> for TowerDefense {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        let mut world = data.world;
        let tower_sheet = decompile_as_sprites(&mut world, "towers.png", (512.0,64.0), (64.0,64.0), 0,false);
        let bullet_sheet = decompile_as_sprites(&mut world, "bullets.png", (64.0,64.0), (32.0,32.0), 1,false);
        let grid_sheet = decompile_as_sprites(&mut world,"tile.png", (192.0, 64.0), (Tile::TILE_SIZE as f32, Tile::TILE_SIZE as f32), 2, true);
        let button_sheet = decompile_as_sprites(&mut world, "buttons.png", (256.0,64.0), (64.0,64.0), 3,false);

        world.register::<CircleCollider>();

        initialise_camera(&mut world);
        initialize_grid(&mut world, grid_sheet.clone(),(150,150));
        initialize_ui(&mut world,button_sheet);
        
        world.add_resource(FPSCounter::new(0));
        world.add_resource(GameTracker::default());
        world.add_resource(Backpack::new(tower_sheet,bullet_sheet,grid_sheet));
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a,'b>,StateEvent>{
        data.data.update(&data.world,true);
    
        if self.fps_display.is_none() {
           data.world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }
        let mut ui_text = data.world.write_storage::<UiText>();
        if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
            if data.world.read_resource::<Time>().frame_number() % 20 == 0 {
                let fps = data.world.read_resource::<FPSCounter>().sampled_fps();
                fps_display.text = format!("FPS: {:.*}", 2, fps);
            }
        }

        Trans::None
    }

}
