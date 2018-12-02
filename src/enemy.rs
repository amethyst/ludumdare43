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
use collider::CircleCollider;

#[derive(Clone,Copy,Debug)]
pub struct Enemy {
    pub health: f32,
}
impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        let health = match enemy_type {
            EnemyType::Normal => EnemyType::NORMAL_HITPOINT,
        };
        Self {
            health,
        }
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: EnemyType::NORMAL_HITPOINT,
        }
    }
}

pub enum EnemyType {
    Normal,
}
impl EnemyType {
    pub const NORMAL_HITPOINT: f32 = 100.0;
}

pub fn initialise_enemy(world: &mut World,sheet_handle: SpriteSheetHandle) {
    let enemy_sprite = SpriteRender {
        sprite_sheet: sheet_handle,
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };

    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width(), dimn.height())
    };
    
    let mut transform = Transform::from(Vector3::new(100.0,64.0,0.0));
    transform.scale = Vector3::new(0.3,0.3,0.0);

    world.create_entity()
                .with(enemy_sprite)
                .with(GlobalTransform::default())
                .with(transform)
                .with(Enemy::default())
                .with(CircleCollider::new(8.0))
                .build();
}

