use bevy::prelude::*;

use crate::action::*;

const Z_MID: f32 = 100.0;
const Z_SCALE: f32 = 0.01;


pub fn z_order(
    mut query: Query<&mut Transform, With<MoveAction>>
) {
    for mut transform in query.iter_mut() {
        transform.translation.z = Z_MID - transform.translation.y * Z_SCALE;
    }
}