use rand::Rng;

use bevy::prelude::*;

use crate::meta::*;

pub const REGION: Rect = Rect{
    min: Vec2 { x: -800.0, y: -400.0 },
    max: Vec2 { x: 800.0, y: 400.0 },
};

const SCALE: Vec3 = Vec3 { x: 0.8, y: 0.8, z: 1.0 };

#[derive(Component)]
pub struct Actor;


#[derive(Bundle)]
pub struct ActorBundle {
    actor: Actor,
    actor_handle: Handle<ActorMeta>,

    #[bundle]
    transform_bundle: TransformBundle,
}


impl ActorBundle {

    pub fn new(spawn_meta: &ActorSpawnMeta) -> Self {

        let mut rng = rand::thread_rng();

        let position = Vec3 {
            x: rng.gen_range(REGION.min.x..REGION.max.x), 
            y: rng.gen_range(REGION.min.y..REGION.max.y),
            z: 0.0,
        };

        let transform = Transform {
            translation: position,
            scale: SCALE,
            ..default()
        };

        let transform_bundle = TransformBundle::from_transform(transform);

        let actor_handle = spawn_meta.actor_handle.clone();

        ActorBundle {
            actor: Actor,
            actor_handle,
            transform_bundle,
        }
    }
}

