use std::collections::HashMap;
use serde::Deserialize;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
};


#[derive(Resource, Deref, DerefMut)]
pub struct LevelHandle(pub Handle<LevelMeta>);


#[derive(Resource, TypeUuid, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[uuid = "afa51a11-f32d-4ee4-811f-1235ced57dfb"]
pub struct LevelMeta {
    pub spawns: Vec<ActorSpawnMeta>,
}


#[derive(TypeUuid, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[uuid = "265d4cd7-0e59-4a32-ba72-76e5969ec993"]
pub struct ActorSpawnMeta {
    pub actor: String,
    pub count: u16,

    #[serde(skip)]
    pub actor_handle: Handle<ActorMeta>,
}


#[derive(Resource, Deref, DerefMut)]
pub struct ActorHandle(pub Handle<ActorMeta>);


#[derive(Component, TypeUuid, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[uuid="a9827c4c-017d-4394-9669-8e9f0cdf8632"]
pub struct ActorMeta {
    pub name: String,
    pub sprite_sheet: ActorSpriteSheetMeta,
}


#[derive(TypeUuid, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[uuid = "9de94c5b-d703-4dd7-b7d8-6819901f6608"]
pub struct ActorSpriteSheetMeta {
    pub image: String,
    pub tile_size: UVec2,
    pub columns: usize,
    pub rows: usize,
    pub fps: f32,
    pub animations: HashMap<String, ClipMeta>,

    #[serde(skip)]
    pub atlas_handle: Handle<TextureAtlas>,
}


#[derive(TypeUuid, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[uuid = "36dcaf0b-5800-4fdc-b71e-53ce778fa1b4"]
pub struct ClipMeta {
    #[serde(skip)]
    pub name: String,

    pub start: usize,
    pub end: usize,

    #[serde(default)]
    pub repeat: bool,
}

