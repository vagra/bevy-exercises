use bevy::prelude::*;
use bevy::diagnostic::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod action;
mod camera;
mod hero;
mod info;

use crate::{
    action::*,
    camera::*,
    hero::*,
    info::*,
};

use common::{
    *,
    assets::*,
    animation::*
};


const BG_COLOR: Color = Color::rgb(0.31, 0.47, 0.51);
const ASSETS_PATH: &str = "../assets/";



fn main() {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins
            .set(AssetPlugin {
                file_path: ASSETS_PATH.to_string(),
                ..default()
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
        ).chain().run_if(in_state(GameState::Playing))
    );
    
    app.run();

}