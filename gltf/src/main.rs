use bevy::core_pipeline::clear_color::ClearColor;
use bevy::diagnostic::*;
use bevy::math::*;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::f32::consts::*;
use std::time::Duration;

const ASSETS_PATH: &str = "../assets/";
const FONT_TTF: &str = "fonts/FiraCode-Regular.ttf";
const MDL_GLTF: &str = "gltf/kid.gltf";
const TIME_STEP: f32 = 1.0 / 60.0;
const RUN_SPEED: f32 = 0.5;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            asset_folder: ASSETS_PATH.to_string(),
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EntityCountDiagnosticsPlugin::default())
        .add_startup_system(setup_scene)
        .add_startup_system(setup_info)
        .add_systems(
            (
                update_info,
                keyboard_control,
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .run();
}

#[derive(Component)]
struct Actor;


#[derive(Component)]
struct Weapon;

#[derive(Component)]
struct Info;

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

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
        mesh: meshes.add(shape::Plane::from_size(200.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // gltf - mdl
    let actor: Handle<Scene> = asset_server.load(format!("{MDL_GLTF}#Scene0"));

    commands.spawn((
        (SceneBundle {
            scene: actor,

            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::new(0.2, 0.2, 0.2),
                ..default()
            },

            ..default()
        }),
        Actor,
    ));

    
}


fn keyboard_control(
    mut query: Query<(
        &Name,
        &mut AnimationPlayer,
        &mut Transform,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<Animations>,
) {
    for (name, mut player, mut transform) in query.iter_mut() {

        if name.as_str() != "GltfNode40" {
            continue;
        }

        let mut run: bool = false;

        if let Some(dir) = get_dir(
            keyboard_input.pressed(KeyCode::Up),
            keyboard_input.pressed(KeyCode::Down),
            keyboard_input.pressed(KeyCode::Left),
            keyboard_input.pressed(KeyCode::Right),
        ) {
            let angle: f32 = dir as f32 * PI * 0.25;
            let offset: Vec3 = - RUN_SPEED * transform.forward();
            transform.rotation = Quat::from_rotation_y(angle);
            transform.translation.x += offset.x;
            transform.translation.z += offset.z;
            player
                .play_with_transition(
                    animations.0[3].clone_weak(),
                    Duration::from_millis(100),
                )
                .set_speed(0.5)
                .repeat();
        }
        else {
            player
                .play_with_transition(
                    animations.0[1].clone_weak(),
                    Duration::from_millis(500),
                )
                .set_speed(0.5)
                .repeat();
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
            position: UiRect {
                right: Val::Px(8.0),
                top: Val::Px(8.0),
                ..default()
            },
            ..default()
        }),
        Info,
    ));
}

fn update_info(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<Info>>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            let mut text = query.single_mut();
            text.sections[2].value = format!("{value:.0}");
        }
    }

    if let Some(num) = diagnostics.get(EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
        if let Some(value) = num.value() {
            let mut text = query.single_mut();
            text.sections[4].value = format!("{value:.0}");
        }
    }
}


fn get_dir(up:bool, down:bool, left:bool, right:bool) -> Option<usize> {
    let mut l = left;
    let mut r = right;
    let mut u = up;
    let mut d = down;

    if l && r {
        l = false;
        r = false;
    }

    if u && d {
        u = false;
        d = false;
    }

    if l && u {
        return Some(5);
    }

    if u && r {
        return Some(3);
    }

    if r && d {
        return Some(1);
    }

    if d && l {
        return Some(7);
    }

    if l {
        return Some(6);
    }

    if u {
        return Some(4);
    }

    if r {
        return Some(2);
    }

    if d {
        return Some(0);
    }

    return None;
}