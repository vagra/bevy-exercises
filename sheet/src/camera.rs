use bevy::prelude::*;


pub fn make_camera(
    mut commands: Commands
) {
    info!("make Camera");

    commands.spawn(Camera2dBundle::default());

}