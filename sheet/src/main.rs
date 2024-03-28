use bevy::prelude::*;
use bevy::diagnostic::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod action;
mod animation;
mod assets;
mod camera;
mod hero;
mod info;

use crate::{
    action::*,
    animation::*,
    assets::*,
    camera::*,
    hero::*,
    info::*,
};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);
const ASSETS_PATH: &str = "../assets/";
const BASE_PATH: &str = "sheet/";
const FONT_TTF: &str = "fonts/FiraCode-Regular.ttf";
const ACTOR_NUM: i32 = 17;
const SPAWN_NUM: i32 = 200;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}


fn main() {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins
            .set(AssetPlugin {
                file_path: ASSETS_PATH.to_string(),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin::default());

    app.insert_resource(ClearColor(BG_COLOR));
    app.init_state::<GameState>();

    app.init_asset::<ActorAsset>()
        .init_asset_loader::<ActorLoader>();

    load_actors(&mut app);
    load_fonts(&mut app);

    app.add_systems(Startup,
        (
            make_info,
            make_camera
        )
    );

    app.add_systems(Update,
        (make_heros).run_if(in_state(GameState::Loading))
    );

    app.add_systems(FixedUpdate,
        (
            random,
            backing,
            moving,
            animating,
            update_info
        ).run_if(in_state(GameState::Playing))
    );
    
    app.run();

}


fn load_actors(app: &mut App) {

    let asset_server = app.world.get_resource::<AssetServer>().unwrap();

    let mut actor_handles = Vec::new();

    for i in 0..ACTOR_NUM {
        let actor_yaml = format!("{BASE_PATH}/actor-{}.actor.yaml", i);
        let actor_handle: Handle<ActorAsset> = asset_server.load(actor_yaml);
        actor_handles.push(ActorHandle(actor_handle));
    }

    app.world.insert_resource(ActorHandles(actor_handles));

}


fn load_fonts(app: &mut App) {

    let asset_server = app.world.get_resource::<AssetServer>().unwrap();
    let font_handle: Handle<Font> = asset_server.load(FONT_TTF);
    app.world.insert_resource(FontHandle(font_handle));
}