use std::collections::HashMap;
use serde::Deserialize;

use bevy::{
    prelude::*,
    reflect::TypePath,
};


#[derive(Resource, Deref, DerefMut)]
pub struct LevelHandle(pub Handle<LevelMeta>);


#[derive(Resource, TypePath, Deserialize, Clone, Debug, Asset)]
#[serde(deny_unknown_fields)]
pub struct LevelMeta {
    pub spawns: Vec<ActorSpawnMeta>,
}


#[derive(TypePath, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActorSpawnMeta {
    pub actor: String,
    pub count: u32,

    #[serde(skip)]
    pub actor_handle: Handle<ActorMeta>,
}


#[derive(Resource, Deref, DerefMut)]
pub struct ActorHandle(pub Handle<ActorMeta>);


#[derive(Component, TypePath, Deserialize, Clone, Debug, Asset)]
#[serde(deny_unknown_fields)]
pub struct ActorMeta {
    pub id: u32,
    pub name: String,
    pub sprite_sheet: ActorSpriteSheetMeta,
}


#[derive(TypePath, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActorSpriteSheetMeta {
    pub image: String,
    pub tile_size: UVec2,
    pub columns: usize,
    pub rows: usize,
    pub fps: f32,
    pub animations: HashMap<String, ClipMeta>,

    #[serde(skip)]
    pub atlas_handle: Handle<TextureAtlasLayout>,
}


#[derive(TypePath, Deserialize, Default, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClipMeta {
    #[serde(skip)]
    pub name: String,

    pub frames: Vec<usize>,

    #[serde(default)]
    pub repeat: bool,
}
