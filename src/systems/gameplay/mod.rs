use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;

pub mod targeting;
mod collision;
mod enemy_removal;
mod score_income;
mod grid_systems;
mod bullets_system;

use self::targeting::TargetingSystem;
use self::bullets_system::{  
    bullet_creation::BulletCreationSystem, 
    bullet_movement::BulletMovementSystem,
    bullet_removal::BulletRemovalSystem,
};
use self::collision::CollisionCheckingSystem;
use self::enemy_removal::EnemyRemovalSystem;
use self::score_income::ScoreIncomeSystem;
use self::grid_systems::tile_selection::TileSelectionSystem;
use self::grid_systems::tower_placement::TowerPlacementSystem;

pub struct TowerDefenseBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for TowerDefenseBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(TargetingSystem, "acquire_target_system", &[]);
        builder.add(BulletCreationSystem::default(),"bullet_creation", &["acquire_target_system"]);
        builder.add(BulletMovementSystem, "bullet_movement", &[]);
        builder.add(CollisionCheckingSystem, "collision_system", &[]);
        builder.add(BulletRemovalSystem, "bullet_removal", &["collision_system"]);
        builder.add(ScoreIncomeSystem, "scroll_income", &[]);
        builder.add(EnemyRemovalSystem, "enemy_removal", &["scroll_income"]);
        builder.add(TileSelectionSystem::default(),"tile_selection", &[]);
        builder.add(TowerPlacementSystem::default(),"tower_placement", &[]);
        Ok(())
    }
}