use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteExpect,ReadStorage};
use amethyst::core::transform::Transform;

use enemy::Enemy;
use tracker::GameTracker;
use super::targeting::acquire_distance;

pub struct ScoreIncomeSystem;

impl<'s> System<'s> for ScoreIncomeSystem {
    type SystemData = (
        ReadStorage<'s,Enemy>,
        WriteExpect<'s,GameTracker>,
    );
    
    fn run(&mut self, (enemies,mut game_tracker) : Self::SystemData) {
        game_tracker.score += (enemies).join().filter(|e| e.health <= 0.0).collect::<Vec<_>>().len() as u32 * 10;
    }
}
