use amethyst::shred::System;
use amethyst::ecs::prelude::{Join,Read,WriteStorage,ReadStorage,Resources};
use amethyst::core::transform::{Transform,GlobalTransform};
use amethyst::renderer::{Camera,VirtualKeyCode};
use amethyst::input::InputHandler;
use amethyst::core::timing::Time;
use amethyst::core::shrev::EventChannel;
use amethyst::core::timing::Stopwatch;

use std::time::Duration;

const CAMERA_ZOOM_SPEED: f32 = 3.0;

pub struct CameraZoomSystem {
    timer: Stopwatch,
}

impl Default for CameraZoomSystem {
    fn default() -> Self {
        CameraZoomSystem {
            timer: Stopwatch::new()
        }
    }
}

impl<'s> System<'s> for CameraZoomSystem {
    type SystemData = (
        ReadStorage<'s,Camera>,
        WriteStorage<'s,Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s,Time>,
    );
    fn setup(&mut self, _res: &mut Resources) {
        self.timer.start();
    }

    fn run(&mut self, (camera, mut transforms,input,time) : Self::SystemData) {
        if self.timer.elapsed() >= Duration::from_millis(200) {
            if let Some((_,t)) = (&camera, &mut transforms).join().next() {
                t.translate_z(1.0);
            }
            self.timer.restart();
        }
    }
}