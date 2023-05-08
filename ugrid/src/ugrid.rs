#![allow(dead_code)]

use bevy::{
    prelude::*,
    sprite::Anchor,
    reflect::TypeUuid,
};

use grid::{*, ugrid::*};

use crate::*;

const GRID_COLOR: Color = Color::rgba(0.75, 0.35, 0.25, 0.4);

const AGENT_RADIUS: u16 = 5;
const CELL_RADIUS: u16 = 30;
const HALF_COLS:u16 = 40;
const HALF_ROWS:u16 = 40;
const COLS:u16 = HALF_COLS * 2;
const ROWS:u16 = HALF_ROWS * 2;


#[derive(Resource, Deref, DerefMut, TypeUuid)]
#[uuid = "e458f087-eee5-48ee-bc11-f59f8826d4ae"]
pub struct Grid(pub UGrid);

impl Default for Grid {
    fn default() -> Self {
        
        Self(UGrid::new(AGENT_RADIUS, CELL_RADIUS, HALF_COLS, HALF_ROWS))
    }
}

#[derive(Component)]
pub struct UCol(pub u16);

#[derive(Component)]
pub struct URow(pub u16);

#[derive(Component)]
pub struct UID(pub u32);

#[derive(Component, Default)]
pub struct UPos{
    pub x: i16,
    pub y: i16,
}

#[derive(Bundle)]
pub struct GridBundle {
    pub col: UCol,
    pub row: URow,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl GridBundle {

    pub fn new(grid:&UGrid, col:u16, row:u16) -> Self {

        let (x, y) = grid.ucell2pos(col, row);

        Self {
            col: UCol(col),
            row: URow(row),

            sprite: SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR.clone(),
                    custom_size: Some(
                            Vec2::new(grid.cell_size as f32, grid.cell_size as f32)
                        ),
                    anchor: Anchor::TopLeft,
                    ..default()
                    }, 
                transform: Transform::from_translation(
                        Vec3::new(x as f32, y as f32, 0.0)
                    ),
                ..default()
            }
        }

    }
}

pub fn make_grids(
    mut commands: Commands,
    grid: Res<Grid>,
) {
    info!("make grid cells...");

    for row in 0..ROWS{
        for col in 0..COLS {
            commands.spawn(GridBundle::new(&grid, col, row));
        }
    }

    commands.insert_resource(NextState(Some(GameState::Infoing)));
}


pub fn update_grids(
    mut query: Query<(
        &UCol,
        &URow,
        &mut Visibility,
    )>,
    grid: ResMut<Grid>,
) {
    for (col, row, mut visibility) in query.iter_mut() {

        if grid.cells[row.0][col.0].head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;
        }
    }


}