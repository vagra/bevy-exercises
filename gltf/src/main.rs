use bevy::render::camera::ClearColor;
use bevy::diagnostic::*;
use bevy::math::{*, primitives::Plane3d};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::*;

mod actor;

use actor::*;


const ASSETS_PATH: &str = "../assets/";
const FONT_TTF: &str = "fonts/FiraCode-Regular.ttf";
const MDL_GLTF: &str = "gltf/kid.gltf";
const TIME_STEP: f64 = 1.0 / 60.0;


fn main() {

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .insert_resource(Time::<Fixed>::from_seconds(TIME_STEP))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: ASSETS_PATH.to_string(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, setup_info)
        .add_systems(FixedUpdate, (update_info, keyboard_control))
        .run();
}

#[derive(Component)]
struct Info;

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // animation
    commands.insert_resource(Animations(vec![
        asset_server.load(format!("{MDL_GLTF}#Animation0")),
        asset_server.load(format!("{MDL_GLTF}#Animation1")),
        asset_server.load(format!("{MDL_GLTF}#Animation2")),
        asset_server.load(format!("{MDL_GLTF}#Animation3")),
        asset_server.load(format!("{MDL_GLTF}#Animation4")),
        asset_server.load(format!("{MDL_GLTF}#Animation5")),
        asset_server.load(format!("{MDL_GLTF}#Animation6")),
        asset_server.load(format!("{MDL_GLTF}#Animation7")),
        asset_server.load(format!("{MDL_GLTF}#Animation8")),
        asset_server.load(format!("{MDL_GLTF}#Animation9")),
        asset_server.load(format!("{MDL_GLTF}#Animation10")),
        asset_server.load(format!("{MDL_GLTF}#Animation11")),
        asset_server.load(format!("{MDL_GLTF}#Animation12")),
        asset_server.load(format!("{MDL_GLTF}#Animation13")),
        asset_server.load(format!("{MDL_GLTF}#Animation14")),
        asset_server.load(format!("{MDL_GLTF}#Animation15")),
        asset_server.load(format!("{MDL_GLTF}#Animation16")),
        asset_server.load(format!("{MDL_GLTF}#Animation17")),
        asset_server.load(format!("{MDL_GLTF}#Animation18")),
        asset_server.load(format!("{MDL_GLTF}#Animation19")),
        asset_server.load(format!("{MDL_GLTF}#Animation20")),
        asset_server.load(format!("{MDL_GLTF}#Animation21")),
        asset_server.load(format!("{MDL_GLTF}#Animation22")),
        asset_server.load(format!("{MDL_GLTF}#Animation23")),
        asset_server.load(format!("{MDL_GLTF}#Animation24")),
        asset_server.load(format!("{MDL_GLTF}#Animation25")),
        asset_server.load(format!("{MDL_GLTF}#Animation26")),
        asset_server.load(format!("{MDL_GLTF}#Animation27")),
    ]));

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    },));

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadow_depth_bias: 0.2,
            shadow_normal_bias: 0.3,
            shadows_enabled: true,
            ..default()
        },

        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX, 0.0, PI / 2., -PI / 4.,
        )),

        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 20.0,
            maximum_distance: 40.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(200.0, 200.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // gltf - mdl
    let model: Handle<Scene> = asset_server.load(format!("{MDL_GLTF}#Scene0"));

    commands.spawn((
        (SceneBundle {
            scene: model,

            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::new(0.2, 0.2, 0.2),
                ..default()
            },

            ..default()
        }),
        Actor::default()
    ));
}


fn keyboard_control(
    mut actor_query: Query<(
        &mut Actor,
        &mut Transform,
    )>,
    mut player_query: Query<
        &mut AnimationPlayer,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    animations: Res<Animations>,
) {
    for (mut actor, mut transform) in actor_query.iter_mut() {
        if let Ok(mut player) = player_query.get_single_mut() {
            
            if let Some(dir) = key2dir(
                keyboard_input.pressed(KeyCode::ArrowLeft),
                keyboard_input.pressed(KeyCode::ArrowRight),
                keyboard_input.pressed(KeyCode::ArrowUp),
                keyboard_input.pressed(KeyCode::ArrowDown),
            ) {
                actor.set_dir(dir);
                actor.run(&mut player, &animations);
                actor.moving();
                transform.translation = actor.position;
            }
            else {
                actor.stand(&mut player, &animations);
            }

            actor.turning();
            transform.rotation = Quat::from_rotation_y(actor.curr_redian());
        }
    }
}


fn setup_info(mut commands: Commands, asset_server: Res<AssetServer>) {
    // font
    let font = asset_server.load(FONT_TTF);
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::GRAY,
    };

    // info
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("scene info", text_style.clone()),
            TextSection::new("\nfps: ", text_style.clone()),
            TextSection::from_style(text_style.clone()), //2
            TextSection::new("\nnum: ", text_style.clone()),
            TextSection::from_style(text_style.clone()), //4
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(8.0),
            top: Val::Px(8.0),
            ..default()
        }),
        Info,
    ));
}


fn update_info(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<Info>>) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            let mut text = query.single_mut();
            text.sections[2].value = format!("{value:.0}");
        }
    }

    if let Some(num) = diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
        if let Some(value) = num.value() {
            let mut text = query.single_mut();
            text.sections[4].value = format!("{value:.0}");
        }
    }
}


fn key2dir(l:bool, r:bool, u:bool, d:bool) -> Option<usize> {
    let mut li = l as usize;
    let mut ri = r as usize;
    let mut ui = u as usize;
    let mut di = d as usize;

    if l && r {
        li = 0;
        ri = 0;
    }

    if u && d {
        ui = 0;
        di = 0;
    }

    let pos: usize = (di << 3) + (li << 2) + (ui << 1) + ri;

    match pos {
        //dlur
        0b0001 => Some(2),
        0b0010 => Some(4),
        0b0100 => Some(6),
        0b1000 => Some(0),
        0b0011 => Some(3),
        0b0110 => Some(5),
        0b1100 => Some(7),
        0b1001 => Some(1),
        _ => None,
    }

}