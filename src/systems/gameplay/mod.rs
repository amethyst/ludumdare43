use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::ecs::prelude::DispatcherBuilder;

pub mod targeting;
mod bullet_creation;
mod bullet_movement;
mod bullet_removal;

use self::targeting::TargetingSystem;
use self::bullet_creation::BulletCreationSystem;
use self::bullet_movement::BulletMovementSystem;
use self::bullet_removal::BulletRemovalSystem;

pub struct TowerDefenseBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for TowerDefenseBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(TargetingSystem, "acquire_target_system", &[]);
        builder.add(BulletCreationSystem::default(),"bullet_creation", &["acquire_target_system"]);
        builder.add(BulletMovementSystem, "bullet_movement", &[]);
        builder.add(BulletRemovalSystem, "bullet_removal", &[]);
        Ok(())
    }
}