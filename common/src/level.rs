
use bevy::prelude::*;

use crate::{
    *,
    meta::*,
    bundle::*,
};


pub fn load_level(
    mut commands: Commands,
    level_handle: Res<LevelHandle>,
    level_assets: Res<Assets<LevelMeta>>,
) {
    info!("load Level...");

    if let Some(level) = level_assets.get(&level_handle) {
        for spawn in &level.spawns {
            info!("load {}  count: {}", spawn.actor, spawn.count);
            
            commands.spawn(ActorBundle::new(spawn));
        }

        commands.insert_resource(level.clone());
        commands.insert_resource(NextState(Some(GameState::Spawning)));
    } else {
        trace!("Awaiting level load");
    }
}