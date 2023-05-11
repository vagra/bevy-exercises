use std::path::*;

use bevy::{
    prelude::*,
    asset::*,
};

use common::meta::*;


pub fn register(app: &mut App) {
    app.register_type::<TextureAtlasSprite>()
        .add_asset::<LevelMeta>()
        .add_asset_loader(LevelMetaLoader)
        .add_asset::<ActorMeta>()
        .add_asset_loader(ActorLoader);
}


pub struct LevelMetaLoader;

impl AssetLoader for LevelMetaLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {

        Box::pin(async move {
            
            let mut meta: LevelMeta = serde_yaml::from_slice(bytes)?;
            info!("loaded Level asset");

            let self_path = load_context.path();

            let mut dependencies = Vec::new();

            for spawn in &mut meta.spawns {

                let (actor_path, actor_handle) = get_relative_asset(
                    load_context, self_path, &spawn.actor);
                dependencies.push(actor_path);

                spawn.actor_handle = actor_handle;
            }

            load_context
                .set_default_asset(LoadedAsset::new(meta)
                .with_dependencies(dependencies));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["level.yml", "level.yaml"]
    }
}


pub struct ActorLoader;

impl AssetLoader for ActorLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {

        Box::pin(async move {

            let mut meta: ActorMeta = serde_yaml::from_slice(bytes)?;
            info!("loaded Actor asset {}", meta.name);

            let self_path = load_context.path();

            let (texture_path, texture_handle) = get_relative_asset(
                load_context, self_path, &meta.sprite_sheet.image);
            
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                meta.sprite_sheet.tile_size.as_vec2(),
                meta.sprite_sheet.columns,
                meta.sprite_sheet.rows,
                None, None);

            let atlas_handle = load_context.set_labeled_asset(
                format!("atlas_{}", &meta.sprite_sheet.image).as_str(),
                LoadedAsset::new(texture_atlas)
                    .with_dependency(texture_path)
            );

            meta.sprite_sheet.atlas_handle = atlas_handle;
            
            for (name, clip) in meta.sprite_sheet.animations.iter_mut() {
                clip.name = name.clone();
            }

            load_context.set_default_asset(
                LoadedAsset::new(meta)
            );

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["actor.yml", "actor.yaml"]
    }
}

fn relative_asset_path(asset_path: &Path, relative_path: &str) -> PathBuf {
    let is_relative = !relative_path.starts_with('/');

    if is_relative {
        let base = asset_path.parent().unwrap_or_else(|| Path::new(""));
        base.join(relative_path)
    } else {
        Path::new(relative_path)
            .strip_prefix("/")
            .unwrap()
            .to_owned()
    }
}

fn get_relative_asset<T: Asset>(
    load_context: &LoadContext,
    self_path: &Path,
    relative_path: &str,
) -> (AssetPath<'static>, Handle<T>) {
    let asset_path = relative_asset_path(self_path, relative_path);
    let asset_path = AssetPath::new(asset_path, None);
    let handle = load_context.get_handle(asset_path.clone());

    (asset_path, handle)
}
