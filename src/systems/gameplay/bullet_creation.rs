use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities,Resources};
use amethyst::core::transform::Transform;
use amethyst::core::cgmath::Vector2;
use amethyst::core::timing::Stopwatch;

use std::time::Duration;

use systems::gameplay::targeting::acquire_distance;
use towers::Tower;
use enemy::Enemy;
use bullet::Bullet;

pub struct BulletCreationSystem {
    stopwatch: Stopwatch,
}

impl Default for BulletCreationSystem {
    fn default() -> Self {
        Self {
            stopwatch: Stopwatch::new()
        }
    }
}

impl<'s> System<'s> for BulletCreationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s,Enemy>,
        WriteStorage<'s,Bullet>,
        WriteStorage<'s,Tower>,
        WriteStorage<'s,Transform>,
    );
    fn setup(&mut self, _res: &mut Resources) {
        self.stopwatch.start();
    }
    fn run(&mut self, (entities,enemies,mut bullets,mut towers,mut transforms) : Self::SystemData) {
        let temp_vector = {
            (&enemies, &*entities,&transforms).join().map(|(enemy,entity,t)| (entity.id(),enemy,t.clone())).collect::<Vec<_>>()
        };
        let tower_vector = {
            (&towers,&transforms).join().map(|(tower,trans)| (tower, trans.clone())).collect::<Vec<_>>()
        };
        for (tower,trans) in tower_vector.iter() {
            if Duration::from_millis(tower.fire_rate) <= self.stopwatch.elapsed() {
                if let Some((_,_,enemy_pos)) = temp_vector
                    .iter()
                    .filter(|(id,_,_)| Some(*id) == tower.target)
                    .last()
                    {
                        let dist = acquire_distance(trans.translation, enemy_pos.translation) as f32;
                        let x = ( (trans.translation.x - enemy_pos.translation.x) / dist).acos();
                        let y = ( (trans.translation.y - enemy_pos.translation.y) / dist).asin();

                        let bullet = entities.build_entity()
                            .with(trans.clone(), &mut transforms)
                            .with(Bullet(Vector2::new(x,y)), &mut bullets);
                            //.build();
                        bullet.build();
                    }
                self.stopwatch.restart();
            }
        }
    }
}
