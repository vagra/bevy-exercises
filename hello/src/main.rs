use bevy::prelude::*;
use bevy::diagnostic::*;
use bevy::math::*;
use bevy::render::camera::ClearColor;
use rand::Rng;

const FONT_TTF: &str = "fonts/FiraCode-Regular.ttf";
const GENERAL_ICON: &str = "hello/general.png";
const SOLDIER_ICON: &str = "hello/soldier.png";
const SPRITE_SCALE: Vec3 = Vec3::new(0.2, 0.2, 1.0);
const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_seconds(TIME_STEP as f64))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "../assets".to_string(),
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate,
            (
                update_general,
                update_soldiers,
                update_info,
                drop_old,
            )
        )
        .run();
}


#[derive(Component)]
struct General {
    move_speed: f32,
    rotate_speed: f32,
}

#[derive(Component)]
struct Soldier {
    move_speed: f32,
    rotate_speed: f32,
}

#[derive(Component)]
struct InfoText;


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(
        Camera2dBundle::default()
    );

    
    let font = asset_server.load(FONT_TTF);
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::GRAY,
    };

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("scene info", text_style.clone()),

            TextSection::new("\nfps: ", text_style.clone()),
            TextSection::from_style(text_style.clone()),    //2

            TextSection::new("\nnum: ", text_style.clone()),
            TextSection::from_style(text_style.clone()),    //4

            TextSection::new("\npos: ", text_style.clone()),
            TextSection::from_style(text_style.clone()),    //6

            TextSection::new("\n up: ", text_style.clone()),
            TextSection::from_style(text_style.clone()),    //8
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(8.0),
            top: Val::Px(8.0),
            ..default()
        }),
        InfoText
    ));

    let mut rng = rand::thread_rng();

    let general: General = General { 
                                move_speed: 200.0 * rng.gen_range(0.3..0.6),
                                rotate_speed: f32::to_radians(360.0 * rng.gen_range(-0.2..0.2))
                            };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GOLD,
                ..default()
            },
            texture: asset_server.load(GENERAL_ICON),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: SPRITE_SCALE,
                ..default()
            },
            ..default()
        },
        general
    ));
}

fn update_general(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut General, &mut Transform)>,
    mut pos_query: Query<&mut Text, With<InfoText>>,
) {

    let mut rng = rand::thread_rng();

    let mut text = pos_query.single_mut();

    let (mut general, mut transform) = query.single_mut();
        
    transform.rotate_z(general.rotate_speed * TIME_STEP);
    let forward: Vec3 = *transform.up();

    transform.translation += forward * general.move_speed * TIME_STEP;
    let position = transform.translation;

    let change = rng.gen_range(0..100);
    if change > 95 {
        general.rotate_speed = f32::to_radians(360.0 * rng.gen_range(-0.2..0.2));

        let turn = turn(&position, &forward);
        if turn != 0 {
            transform.rotate_z(f32::to_radians(30.0 * turn as f32));
        }

        text.sections[6].value = format!("{:.0}, {:.0}", position.x, position.y);
        text.sections[8].value = format!("{:.2}, {:.2}", forward.x, forward.y);
    }

    let mut soldier_transform = transform.clone();
    soldier_transform.rotate_z(f32::to_radians(180.0));
    soldier_transform.translation -= forward * 12.0;
    soldier_transform.scale = Vec3::new(0.1, 0.1, 1.0);

    let soldier_texture = asset_server.load(SOLDIER_ICON);

    let soldier_color = Color::Rgba {
                        red: rng.gen_range(0.2..0.9),
                        green: rng.gen_range(0.2..0.9),
                        blue: rng.gen_range(0.2..0.9),
                        alpha: rng.gen_range(0.5..1.0)
                    };

    for _i in 0..80 {
        let soldier: Soldier = Soldier {
            move_speed: 100.0 * rng.gen_range(0.5..2.0),
            rotate_speed: f32::to_radians(360.0 * rng.gen_range(-0.3..0.3))
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: soldier_color,
                    ..default()
                },
                texture: soldier_texture.clone(),
                transform: soldier_transform,
                ..default()
            },
            soldier            
        ));
    }

}

fn update_soldiers(
    mut query: Query<(&Soldier, &mut Transform, &mut Sprite)>,
) {

    let mut rng = rand::thread_rng();

    for (soldier, mut transform, mut sprite) in &mut query {

        transform.rotate_z(soldier.rotate_speed * TIME_STEP);
        let forward: Vec3 = *transform.up();

        transform.translation += forward * soldier.move_speed * TIME_STEP;
        let position = transform.translation;
        
        let change = rng.gen_range(0..100);

        if change > 95 {
            let turn = turn(&position, &forward);
            if turn != 0 {
                transform.rotate_z(f32::to_radians(30.0 * turn as f32));
            }

            let alpha = sprite.color.a() - 0.05;
            if alpha >= 0.0 {
                sprite.color.set_a(alpha);
            }

            if transform.scale.x < 0.2 {
                transform.scale += Vec3::new(0.01, 0.01, 0.0);
            }
        }
    }

}

fn drop_old(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Sprite),  With<Soldier>>,
) {
    let mut rng = rand::thread_rng();

    for (entity, transform, sprite) in &mut query.iter() {

        let change = rng.gen_range(0..100);

        if change > 95 {
            let position = transform.translation;

            if is_overflow(&position)
            || sprite.color.a() <= 0.05 {
                commands.entity(entity).despawn();
            }
        }
    }

}

fn update_info(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<InfoText>>
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            let mut text = query.single_mut();
            text.sections[2].value =  format!("{value:.0}");
        }
    }

    if let Some(num) = diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
        if let Some(value) = num.value() {
            let mut text = query.single_mut();
            text.sections[4].value =  format!("{value:.0}");
        }
    }
}

fn is_overflow(pos: &Vec3) -> bool {
    if pos.x < -400.0
    || pos.x > 400.0
    || pos.y < -300.0
    || pos.y > 300.0 {
        true
    }
    else {
        false
    }
}

fn turn(pos: &Vec3, up: &Vec3) -> i8 {

    if (pos.x < -240.0 && up.x < 0.0 && up.y < 0.0)
    || (pos.x > 240.0 && up.x > 0.0 && up.y > 0.0)
    || (pos.y < -160.0 && up.y < 0.0 && up.x > 0.0)
    || (pos.y > 160.0 && up.y > 0.0 && up.x < 0.0) {
        1
    }
    else if (pos.x < -240.0 && up.x < 0.0 && up.y > 0.0)
    || (pos.x > 240.0 && up.x > 0.0 && up.y < 0.0)
    || (pos.y < -160.0 && up.y < 0.0 && up.x < 0.0)
    || (pos.y > 160.0 && up.y > 0.0 && up.x > 0.0) {
        -1
    }
    else {
        0
    }

}