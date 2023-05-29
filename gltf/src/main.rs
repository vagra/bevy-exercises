use bevy::prelude::*;
use bevy::diagnostic::*;
use bevy::math::*;
use bevy::core_pipeline::clear_color::ClearColor;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
// use rand::Rng;
use std::f32::consts::*;
use std::time::Duration;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const ASSETS_PATH: &str = "../assets/";
const FONT_TTF: &str = "fonts/FiraCode-Regular.ttf";
const MODEL_GLTF: &str = "gltf/kid.gltf";
const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(
            DirectionalLightShadowMap { size: 2048 }
        )
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            asset_folder: ASSETS_PATH.to_string(),
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EntityCountDiagnosticsPlugin::default())
        .add_startup_system(setup_scene)
        .add_startup_system(setup_info)
        .add_system(loaded)
        .add_systems(
            (
                update_info,
                keyboard_control,
                animate_actor,
            )
            .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .run();
}


#[derive(Component)]
struct Actor;


#[derive(Component)]
struct Info;


#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);


fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {

    // animation
    commands.insert_resource(Animations(vec![
        asset_server.load(format!("{MODEL_GLTF}#Animation0")),
        asset_server.load(format!("{MODEL_GLTF}#Animation1")),
        asset_server.load(format!("{MODEL_GLTF}#Animation2")),
        asset_server.load(format!("{MODEL_GLTF}#Animation3")),
        asset_server.load(format!("{MODEL_GLTF}#Animation4")),
        asset_server.load(format!("{MODEL_GLTF}#Animation5")),
        asset_server.load(format!("{MODEL_GLTF}#Animation6")),
        asset_server.load(format!("{MODEL_GLTF}#Animation7")),
        asset_server.load(format!("{MODEL_GLTF}#Animation8")),
        asset_server.load(format!("{MODEL_GLTF}#Animation9")),
        asset_server.load(format!("{MODEL_GLTF}#Animation10")),
        asset_server.load(format!("{MODEL_GLTF}#Animation11")),
        asset_server.load(format!("{MODEL_GLTF}#Animation12")),
        asset_server.load(format!("{MODEL_GLTF}#Animation13")),
        asset_server.load(format!("{MODEL_GLTF}#Animation14")),
        asset_server.load(format!("{MODEL_GLTF}#Animation15")),
        asset_server.load(format!("{MODEL_GLTF}#Animation16")),
        asset_server.load(format!("{MODEL_GLTF}#Animation17")),
        asset_server.load(format!("{MODEL_GLTF}#Animation18")),
        asset_server.load(format!("{MODEL_GLTF}#Animation19")),
        asset_server.load(format!("{MODEL_GLTF}#Animation20")),
        asset_server.load(format!("{MODEL_GLTF}#Animation21")),
        asset_server.load(format!("{MODEL_GLTF}#Animation22")),
        asset_server.load(format!("{MODEL_GLTF}#Animation23")),
        asset_server.load(format!("{MODEL_GLTF}#Animation24")),
        asset_server.load(format!("{MODEL_GLTF}#Animation25")),
        asset_server.load(format!("{MODEL_GLTF}#Animation26")),
        asset_server.load(format!("{MODEL_GLTF}#Animation27")),
    ]));

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 5.0, 10.0)
                .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
            ..default()
        },
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },

        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            shape::Plane::from_size(10.0).into()
        ),
        material: materials.add(
            Color::rgb(0.3, 0.5, 0.3).into()
        ),
        ..default()
    });

    // gltf
    commands.spawn(((
        SceneBundle {
            scene: asset_server.load(format!("{MODEL_GLTF}#Scene0")),
            ..default()
        }),
        Actor
    ));
    
}



// Once the scene is loaded, start the animation
fn loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {

            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}



fn keyboard_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut action: Local<usize>,
) {

    if let Ok(mut player) = player.get_single_mut() {

        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            *action = (*action + 1) % animations.0.len();
            player
                .play_with_transition(
                    animations.0[*action].clone_weak(),
                    Duration::from_millis(500),
                )
                .repeat();
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            *action = (*action + animations.0.len() - 1) % animations.0.len();
            player
                .play_with_transition(
                    animations.0[*action].clone_weak(),
                    Duration::from_millis(500),
                )
                .repeat();
        }
    }
}


fn animate_actor(
    time: Res<Time>,
    mut query: Query<
        &mut Transform,
        With<Actor>
    >,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            0.0,
        );
    }
}



fn setup_info(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    
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
            TextSection::from_style(text_style.clone()),    //2

            TextSection::new("\nnum: ", text_style.clone()),
            TextSection::from_style(text_style.clone()),    //4
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(8.0),
                top: Val::Px(8.0),
                ..default()
            },
            ..default()
        }),
        Info
    ));
    
}


fn update_info(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<Info>>
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            let mut text = query.single_mut();
            text.sections[2].value =  format!("{value:.0}");
        }
    }

    if let Some(num) = diagnostics.get(EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
        if let Some(value) = num.value() {
            let mut text = query.single_mut();
            text.sections[4].value =  format!("{value:.0}");
        }
    }
}


