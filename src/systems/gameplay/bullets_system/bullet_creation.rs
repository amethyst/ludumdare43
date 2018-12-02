use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,ReadExpect,Entities,Resources};
use amethyst::core::transform::{GlobalTransform,Transform};
use amethyst::core::cgmath::Vector2;
use amethyst::core::timing::Stopwatch;
use amethyst::renderer::SpriteRender;
use amethyst::core::transform::components::Parent;

use std::time::Duration;

use towers::{Tower,Element};
use enemy::Enemy;
use bullet::Bullet;
use backpack::Backpack;
use collider::CircleCollider;


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
        ReadExpect<'s,Backpack>,
        Entities<'s>,
        ReadStorage<'s,Enemy>,
        WriteStorage<'s,Bullet>,
        WriteStorage<'s,Tower>,
        WriteStorage<'s,Transform>,
        WriteStorage<'s,GlobalTransform>,
        WriteStorage<'s,SpriteRender>,
        WriteStorage<'s,CircleCollider>,
        WriteStorage<'s,Parent>,
    );
    fn setup(&mut self, _res: &mut Resources) {
        self.stopwatch.start();
    }
    fn run(&mut self, (bp,
            entities,enemies,
            mut bullets,
            mut towers,
            mut transforms,
            mut gtransforms,
            mut sprites,
            mut colliders,
            mut parents) : Self::SystemData) {

        let temp_vector = {
            (&enemies, &*entities,&transforms).join().map(|(enemy,entity,t)| (entity.id(),enemy,t.clone())).collect::<Vec<_>>()
        };
        let tower_vector = {
            (&towers,&*entities,&transforms).join().map(|(tower,entity,trans)| (tower,entity,trans.clone())).collect::<Vec<_>>()
        };
        for (tower,entity,trans) in tower_vector.iter() {
            if Duration::from_millis(tower.fire_rate) <= self.stopwatch.elapsed() {
                if let Some((_,_,enemy_pos)) = temp_vector
                    .iter()
                    .filter(|(id,_,_)| Some(*id) == tower.target)
                    .last()
                    {
                        let angle = (enemy_pos.translation.y - trans.translation.y).atan2(enemy_pos.translation.x - trans.translation.x);
                        let x = angle.cos();
                        let y = angle.sin();
                        
                        let sheet_handle = bp.bullet_sheet.clone().unwrap();

                        let sprite = SpriteRender {
                            sprite_sheet: sheet_handle,
                            sprite_number: 0,
                            flip_horizontal: false,
                            flip_vertical: false,
                        };
                        entities.build_entity()
                            .with(trans.clone(), &mut transforms)
                            .with(GlobalTransform::default(),&mut gtransforms)
                            .with(sprite, &mut sprites)
                            .with(Bullet::new(Vector2::new(x,y),Element::DEFAULT_BULLET_DAMAGE), &mut bullets)
                            .with(CircleCollider::new(8.0), &mut colliders)
                            //.with(Parent { entity: *entity }, &mut parents)
                            .build();
                    }
                self.stopwatch.restart();
            }
        }
    }
}
