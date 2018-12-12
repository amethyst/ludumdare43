use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,Read,WriteStorage,ReadStorage,ReadExpect};
use amethyst::core::transform::{Transform,GlobalTransform};
use amethyst::renderer::{Camera,VirtualKeyCode,ScreenDimensions};
use amethyst::input::InputHandler;
use amethyst::core::timing::Time;


const CAMERA_SPEED: f32 = 4.0;
const CAMERA_ACTIVATE: f64 = 5.0;

pub struct CameraMovementSystem;


impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        ReadStorage<'s,Camera>,
        WriteStorage<'s,Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s,Time>,
        ReadExpect<'s,ScreenDimensions>,
    );
    
    fn run(&mut self, (camera, mut transforms,input,time,dims) : Self::SystemData) {
        if let Some((dx,dy)) = input.mouse_position() {
            if let Some((_,trans)) = (&camera,&mut transforms).join().next() {
                let (width,height) = ( dims.width() as f64,dims.height() as f64);
                if x <= CAMERA_ACTIVATE &&  y <= CAMERA_ACTIVATE {
                    trans.translate_x(-CAMERA_SPEED);
                    trans.translate_y(CAMERA_SPEED);
                // Top left
                } else if x <= CAMERA_ACTIVATE && y >= height - CAMERA_ACTIVATE {
                    trans.translate_x(-CAMERA_SPEED);
                    trans.translate_y(-CAMERA_SPEED);
                // Bottom right
                } else if x >= width - CAMERA_ACTIVATE && y <= CAMERA_ACTIVATE {
                    trans.translate_x(CAMERA_SPEED);
                    trans.translate_y(CAMERA_SPEED);
                // Top right
                } else if x >= width - CAMERA_ACTIVATE && y >= height - CAMERA_ACTIVATE {
                    trans.translate_x(CAMERA_SPEED);
                    trans.translate_y(-CAMERA_SPEED);
                // Left
                } else if x <= CAMERA_ACTIVATE {
                    trans.translate_x(-CAMERA_SPEED);
                // Top
                } else if y <= CAMERA_ACTIVATE {
                    trans.translate_y(CAMERA_SPEED);
                // Right
                } else if x >= width - CAMERA_ACTIVATE {
                    trans.translate_x(CAMERA_SPEED);
                // Bottm y == dims.height()
                } else if y >= height - CAMERA_ACTIVATE {
                    trans.translate_y(-CAMERA_SPEED);
                }

            }
        }
    }
}
