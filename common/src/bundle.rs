use bevy::prelude::*;

use crate::meta::*;


#[derive(Component)]
pub struct SpawnCount(pub u32);


#[derive(Bundle)]
pub struct ActorBundle {
    spawn_count: SpawnCount,
    actor_handle: Handle<ActorMeta>,
}


impl ActorBundle {

    pub fn new(spawn_meta: &ActorSpawnMeta) -> Self {

        Self {
            spawn_count: SpawnCount(spawn_meta.count),
            actor_handle: spawn_meta.actor_handle.clone(),
        }
    }
}



