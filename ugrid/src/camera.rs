use bevy::prelude::*;

use common::*;

const CAMERA_SPEED: f32 = 5.0;

pub fn make_camera(
    mut commands: Commands
) {
    info!("make Camera");

    commands.spawn(Camera2dBundle::default());

}

pub fn camera_control(
    mut query: Query<
        &mut Transform,
        With<Camera>,
    >,
    input: Res<Input<KeyCode>>
) {
    let mut transform = query.get_single_mut()
                        .expect("error: camera not found.");

    let l = input.pressed(KeyCode::Left);
    let r = input.pressed(KeyCode::Right);
    let u = input.pressed(KeyCode::Up);
    let d = input.pressed(KeyCode::Down);

    if let Some(pos) = key2dir(l, r, u, d) {

        let offset = VECTORES[pos];
        transform.translation.x += CAMERA_SPEED * offset.x;
        transform.translation.y += CAMERA_SPEED * offset.y;
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