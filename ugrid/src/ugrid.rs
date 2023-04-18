#![allow(dead_code)]

use bevy::{
    prelude::*,
    sprite::Anchor,
    reflect::TypeUuid,
};

use grid::{ugrid::*, pool::*};

use crate::*;

const GRID_COLOR: Color = Color::rgba(0.75, 0.35, 0.25, 0.4);

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
pub struct GridRow(u16);

impl GridRow {

    pub fn new(index:u16) -> Self {

        Self(index)
    }
}


#[derive(Component)]
pub struct GridCol(u16);

impl GridCol {

    pub fn new(index:u16) -> Self {

        Self(index)
    }
}

#[derive(Bundle)]
pub struct GridBundle {
    pub col: GridCol,
    pub row: GridRow,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl GridBundle {

    pub fn new(col:u16, row:u16) -> Self {

        let (x, y) = ugrid::cell2pos(col, row);

        Self {
            col: GridCol::new(col),
            row: GridRow::new(row),

            sprite: SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR.clone(),
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    anchor: Anchor::TopLeft,
                    ..default()
                    }, 
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            }
        }

    }
}

pub fn make_grids(
    mut commands: Commands,
) {
    info!("make grid cells...");

    for row in 0..ROWS {
        for col in 0..COLS {
            commands.spawn(GridBundle::new(col, row));
        }
    }

    commands.insert_resource(NextState(Some(GameState::Playing)));

    info!("playing...");
}


pub fn update_grids(
    mut query: Query<(
        &GridCol,
        &GridRow,
        &mut Sprite,
    )>,
    grid: ResMut<Grid>,
) {
    for (col, row, mut sprite) in query.iter_mut() {

        if grid.cells[row.0][col.0].head == INVALID {
            sprite.color = Color::NONE;
        }
        else {
            sprite.color = GRID_COLOR;
        }
    }


}