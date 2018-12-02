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
use rand;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
#[derive(Copy,Clone,Debug)]
pub enum Element {
    Fire = 0,
    Ice =  1,
    Earth = 2,
    Air = 3,
}
impl Distribution<Element> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Element {
        match rng.gen_range(0, 4) {
            0 => Element::Fire,
            1 => Element::Ice,
            2 => Element::Earth,
            3 => Element::Air,
            _ => Element::Fire,
        }
    }
}
impl Element {
    pub const DEFAULT_RANGE: f32 = 100.0;
    pub const DEFAULT_FIRE_RATE: u64 = 1000;
    pub const DEFAULT_BULLET_SPEED: f32 = 100.0;
}

pub struct Tower {
    pub level: u32,
    pub element: Element,
    pub range: f32,
    pub target: Option<u32>,
    pub fire_rate: u64,
}

impl Tower {
    pub fn new(element: Element,range: f32,fire_rate: u64) -> Self {
        Self {
            level: 0,
            element: element,
            range: range,
            target: None,
            fire_rate: fire_rate,
        }
    }
    pub fn upgrade(&mut self) {
        self.level += 1;
    }
}

impl Default for Tower {
    fn default() -> Self {
        Tower {
            level: 0,
            element: Element::Fire,
            range: Element::DEFAULT_RANGE,
            target: None,
            fire_rate: Element::DEFAULT_FIRE_RATE,
        }
    }
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_tower(world: &mut World,sheet_handle: SpriteSheetHandle)  {
    let sprite_id = rand::random::<Element>();

    let tower_sprite = SpriteRender {
        sprite_sheet: sheet_handle,
        sprite_number: sprite_id as usize,
        flip_horizontal: false,
        flip_vertical: false,
    };

    let (width,height) = {
        let dimn = world.read_resource::<ScreenDimensions>();
        (dimn.width(), dimn.height())
    };
    
    let (x,y) = ((width / 2.0) ,(height / 2.0));

    world.create_entity()
                .with(tower_sprite)
                .with(GlobalTransform::default())
                .with(Transform::from(Vector3::new(x,y,0.0)))
                .with(Tower::new(sprite_id, Element::DEFAULT_RANGE,Element::DEFAULT_FIRE_RATE))
                .build();
}