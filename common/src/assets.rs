use std::collections::HashMap;

use bevy::{
    asset::{io::Reader, *}, prelude::*
};
use serde::Deserialize;

use crate::*;


#[derive(Resource)]
pub struct ActorHandles(pub Vec<ActorHandle>);


#[derive(Asset, Resource, Component, TypePath, Deref, DerefMut)]
pub struct ActorHandle(pub Handle<ActorAsset>);


#[derive(Resource, Deref, DerefMut)]
pub struct FontHandle(pub Handle<Font>);


#[derive(Asset, Component, TypePath, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActorAsset {
    pub id: u32,
    pub name: String,
    pub image: String,
    pub tile_size: UVec2,
    pub columns: usize,
    pub rows: usize,
    pub fps: f32,
    pub animations: HashMap<String, ClipMeta>,

    #[serde(skip)]
    pub layout_handle: Handle<TextureAtlasLayout>,
    #[serde(skip)]
    pub image_handle: Handle<Image>,
}


#[derive(TypePath, Deserialize, Default, Clone, Debug, Asset)]
#[serde(deny_unknown_fields)]
pub struct ClipMeta {
    #[serde(skip)]
    pub name: String,

    pub frames: Vec<usize>,

    #[serde(default)]
    pub repeat: bool,
}


#[derive(Default)]
pub struct ActorLoader;


impl AssetLoader for ActorLoader {
    type Asset = ActorAsset;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {

        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut actor: ActorAsset = serde_yaml::from_slice(&bytes)?;
            info!("load actor asset {}", actor.name);
            
            let layout = TextureAtlasLayout::from_grid(
                actor.tile_size.as_vec2(),
                actor.columns,
                actor.rows,
                None, None);

            let layout_handle = load_context.add_labeled_asset(
                format!("layout_{}", &actor.image),
                layout
            );

            actor.layout_handle = layout_handle;

            
            let image_path = format!("{BASE_PATH}/{}", actor.image.clone());

            let image_handle: Handle<Image> = load_context.load(image_path);

            actor.image_handle = image_handle;
            
            
            for (name, clip) in actor.animations.iter_mut() {
                clip.name = name.clone();
            }

            Ok(actor)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["actor.yml", "actor.yaml"]
    }
}



pub fn load_actors(app: &mut App) {

    let asset_server = app.world.get_resource::<AssetServer>().unwrap();

    let mut actor_handles = Vec::new();

    for i in 0..ACTOR_NUM {
        let actor_yaml = format!("{BASE_PATH}/actor-{}.actor.yaml", i);
        let actor_handle: Handle<ActorAsset> = asset_server.load(actor_yaml);
        actor_handles.push(ActorHandle(actor_handle));
    }

    app.world.insert_resource(ActorHandles(actor_handles));

}


pub fn load_fonts(app: &mut App) {

    let asset_server = app.world.get_resource::<AssetServer>().unwrap();
    let font_handle: Handle<Font> = asset_server.load(FONT_TTF);
    app.world.insert_resource(FontHandle(font_handle));
}