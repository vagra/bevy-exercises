use bevy::{
    prelude::*,
    diagnostic::*,
};

mod action;
mod actor;
mod animation;
mod assets;
mod camera;
mod hero;
mod info;
mod level;
mod meta;
mod ugrid;

use crate::{
    action::*,
    animation::*,
    assets::*,
    camera::*,
    hero::*,
    info::*,
    level::*,
    meta::*,
    ugrid::*,
};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);
const ASSETS_PATH: &str = "../assets/";
const LEVEL_YAML: &str = "ugrid/game.level.yaml";
const FONT_TTF: &str = "fonts/FiraCode-Regular.ttf";


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Spawning,
    Griding,
    Playing,
    Paused,
}


fn main() {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins
            .set(AssetPlugin {
                asset_folder: ASSETS_PATH.to_string(),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
        )
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EntityCountDiagnosticsPlugin::default());

    app.insert_resource(ClearColor(BG_COLOR))
        .add_state::<GameState>();

    register(&mut app);

    let asset_server = app.world.get_resource::<AssetServer>().unwrap();
    let level_asset = LEVEL_YAML;
    let level_handle: Handle<LevelMeta> = asset_server.load(level_asset);

    let font_asset = FONT_TTF;
    let font_handle: Handle<Font> = asset_server.load(font_asset);

    app.world.insert_resource(LevelHandle(level_handle));
    app.world.insert_resource(FontHandle(font_handle));

    app.add_startup_system(make_info)
        .add_startup_system(make_camera)
        .add_startup_system(setup);

    app.add_system(
            (load_level).run_if(in_state(GameState::Loading))
        )
        .add_system(
            (make_heros).run_if(in_state(GameState::Spawning))
        )
        // .add_system(
        //     (make_grids).run_if(in_state(GameState::Griding))
        // )
        .add_system(
            (update).in_schedule(CoreSchedule::FixedUpdate),
        )
        .add_system(
            (random).after(update)
            .run_if(in_state(GameState::Playing)),
        )
        .add_system(
            (turning).after(update)
            .run_if(in_state(GameState::Playing)),
        )
        .add_system(
            (moving).after(turning)
            .run_if(in_state(GameState::Playing)),
        )
        // .add_system(
        //     (update_grids).after(turning)
        //     .run_if(in_state(GameState::Playing)),
        // )
        .add_system(
            (animating).after(turning)
            .run_if(in_state(GameState::Playing)),
        )
        .add_system(
            (update_info).after(animating)
            .run_if(in_state(GameState::Playing)),
        );
    
    app.run();

}

fn setup() {
    info!("hello, character sprite sheet!");

}

fn update() {

}