use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod actor;
mod animation;
mod assets;
mod hero;
mod level;
mod meta;

use crate::{
    animation::*,
    assets::*,
    hero::*,
    level::*,
    meta::*,
};



#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Spawning,
    Playing,
    Paused,
}


fn main() {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins
            .set(AssetPlugin {
                asset_folder: "../assets".to_string(),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugin(WorldInspectorPlugin::new());

    app.insert_resource(ClearColor(Color::hex("#507883").unwrap()))
        .add_state::<GameState>();

    register(&mut app);

    let asset_server = app.world.get_resource::<AssetServer>().unwrap();
    let level_asset = "sheet/game.level.yaml";
    let level_handle: Handle<LevelMeta> = asset_server.load(level_asset);

    app.world.insert_resource(LevelHandle(level_handle));

    app.add_startup_system(setup);
    app.add_system(
            (load_level).run_if(in_state(GameState::Loading))
        )
        .add_system(
            (make_heros).run_if(in_state(GameState::Spawning))
        )
        .add_system(
            (update).in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_system(
            (animating).in_schedule(CoreSchedule::FixedUpdate),
        );
    
    app.run();

}

fn setup(
    mut commands: Commands
) {
    info!("hello, yaml!");

    commands.spawn(
        Camera2dBundle::default()
    );
    
}

fn update() {

}