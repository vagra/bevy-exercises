use rand::Rng;

use bevy::prelude::*;

use crate::meta::*;


const SCALE: Vec3 = Vec3{x: 0.8, y: 0.8, z: 1.0};

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
            x: rng.gen_range(-800.0..800.0), 
            y: rng.gen_range(-400.0..400.0),
            z: 0.0
        };

        let transform = Transform {
            translation: position,
            scale: SCALE,
            ..default()
        };

        let transform_bundle = 
            TransformBundle::from_transform(transform);

        let actor_handle = spawn_meta.actor_handle.clone();

        ActorBundle {
            actor: Actor,
            actor_handle,
            transform_bundle,
        }
    }
}

