use rand::Rng;

use bevy::prelude::*;

pub const REGION: Rect = Rect{
    min: Vec2 { x: -800.0, y: -400.0 },
    max: Vec2 { x: 800.0, y: 400.0 },
};

#[derive(Component, Default, Debug)]
pub struct Actor {
    pub index: u32,
    pub id: u32,
    pub prev_x: f32,
    pub prev_y: f32,
    pub x: f32,
    pub y: f32,
}

impl Actor {

    pub fn new(index:u32, id:u32) -> Self {

        let (x, y) = gen_rand_position();
        
        Self {
            index,
            id,
            prev_x: x,
            prev_y: y,
            x,
            y,
        }
    }
}


fn gen_rand_position() -> (f32, f32) {
    let mut rng = rand::thread_rng();

    ( rng.gen_range(REGION.min.x..REGION.max.x),
        rng.gen_range(REGION.min.y..REGION.max.y) )
}
