#![allow(dead_code)]

use bevy::{
    prelude::*,
    sprite::Anchor,
    reflect::TypeUuid,
};

use grid::{ugrid::UGrid};

use crate::*;

const GRID_COLOR: Color = Color::rgba(0.75, 0.35, 0.25, 0.3);
const GRID_SIZE: f32 = 20.0;

const AGENT_RADIUS: f32 = 5.0;


#[derive(Resource, Deref, DerefMut, TypeUuid)]
#[uuid = "e458f087-eee5-48ee-bc11-f59f8826d4ae"]
pub struct Grid(pub UGrid);

impl Default for Grid {
    fn default() -> Self {
        
        Self(UGrid::new(AGENT_RADIUS))
    }
}

#[derive(Component)]
pub struct GridIndex(u16);

impl GridIndex {

    pub fn new(index:u16) -> Self {

        Self(index)
    }
}

#[derive(Bundle)]
pub struct GridBundle {
    pub index: GridIndex,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl GridBundle {

    pub fn new(index:u16, x:i16, y:i16) -> Self {

        Self {
            index: GridIndex::new(index),

            sprite: SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR.clone(),
                    custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                    anchor: Anchor::Center,
                    ..default()
                    }, 
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                ..default()
            }
        }

    }
}

pub fn make_grids(
    mut commands: Commands,
    grid: ResMut<Grid>,
) {
    info!("make grid rects...");

    for index in 0..grid.pool.size {
        let agent = grid.pool[index];

        if agent.is_free() {
            continue;
        }

        commands.spawn(GridBundle::new(index as u16, agent.x, agent.y));
    }

    commands.insert_resource(NextState(Some(GameState::Playing)));

    info!("playing...");
}


pub fn update_grids(
    mut query: Query<(
        &GridIndex,
        &mut Transform,
    )>,
    grid: ResMut<Grid>,
) {
    for (index, mut transform) in query.iter_mut() {

        transform.translation.x = grid.pool[index.0].x as f32;
        transform.translation.y = grid.pool[index.0].y as f32;
    }


}