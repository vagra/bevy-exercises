use rand::Rng;

use bevy::prelude::*;

use crate::meta::*;

pub const REGION: Rect = Rect{
    min: Vec2 { x: -800.0, y: -400.0 },
    max: Vec2 { x: 800.0, y: 400.0 },
};

pub const MIN_AGENT_RADIUS: i16 = 5;
pub const MAX_AGENT_RADIUS: i16 = 10;

#[derive(Component, Default, Debug)]
pub struct Actor {
    pub index: u32,
    pub id: u32,
    pub prev_x: f32,
    pub prev_y: f32,
    pub x: f32,
    pub y: f32,
    pub hw: i16,
    pub hh: i16,
}

impl Actor {

    pub fn new(index:u32, id:u32) -> Self {

        let (x, y) = gen_rand_pos();

        let r = gen_rand_size();
        
        Self {
            index,
            id,
            prev_x: x,
            prev_y: y,
            x,
            y,
            hw: r,
            hh: r,
        }
    }
}


#[derive(Component)]
pub struct SpawnCount(pub u32);


#[derive(Bundle)]
pub struct ActorBundle {
    spawn_count: SpawnCount,
    actor_handle: Handle<ActorMeta>,
}


impl ActorBundle {

    pub fn new(spawn_meta: &ActorSpawnMeta) -> Self {

        ActorBundle {
            spawn_count: SpawnCount(spawn_meta.count),
            actor_handle: spawn_meta.actor_handle.clone(),
        }
    }
}


fn gen_rand_pos() -> (f32, f32) {
    let mut rng = rand::thread_rng();

    ( rng.gen_range(REGION.min.x..REGION.max.x),
        rng.gen_range(REGION.min.y..REGION.max.y) )
}


fn gen_rand_size() -> i16 {
    let mut rng = rand::thread_rng();

    rng.gen_range(MIN_AGENT_RADIUS..MAX_AGENT_RADIUS)
}
