use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,WriteStorage,ReadStorage,Entities};
use amethyst::core::transform::Transform;
use amethyst::core::cgmath::Vector3;

use towers::Tower;
use enemy::Enemy;

pub struct TargetingSystem;

impl<'s> System<'s> for TargetingSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s,Enemy>,
        WriteStorage<'s,Tower>,
        ReadStorage<'s,Transform>,
    );
    
    fn run(&mut self, (entities,enemies,mut towers,transforms) : Self::SystemData) {
        let enemies_vector = {
            (&enemies,&transforms,&*entities).join()
                .map(|(_,t,entitiy)| (entitiy.id(), t.clone()) )
                .collect::<Vec<_>>()
        };

        for (tower,transform) in (&mut towers,&transforms).join() {
            //if let None = tower.target {
            //    continue;
            //}
            let origin = transform.translation;
            let range = tower.range;
            
            // Getting the distance between the enemy and the tower
            // and filtering enemies in range.
            let mut temp_vector = enemies_vector
                .iter()
                .map(|(id,t)| (*id,acquire_distance(origin, t.translation)) )
                .filter(|(_,distance)| enemy_is_in_range(*distance as f32,range))
                .collect::<Vec<_>>();

            // Sorting the vector based on distance.
            temp_vector.sort_by_key(|(_,dist)| *dist );

            // Getting the closest enemy and setting its 
            // id as the target id.
            if let Some(idx) = temp_vector
                .iter()
                .map(|(id, _)| *id)
                .last()
                {
                    tower.target = Some(idx);
                } else {
                    tower.target = None;
                }
        }
    }
}

fn enemy_is_in_range(distance: f32, range: f32) -> bool {
    if distance <= range {
        return true;
    }
    false 
}

pub fn acquire_distance(v1: Vector3<f32>, v2: Vector3<f32>) -> u32 {
    ( (v1.x - v2.x).powf(2.0) + (v1.y - v2.y).powf(2.0) ).sqrt() as u32
}