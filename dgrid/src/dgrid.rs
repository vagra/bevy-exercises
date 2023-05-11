

use bevy::{
    prelude::*,
    sprite::Anchor,
    reflect::TypeUuid,
};

use grid::{*, dgrid::{*, lcell::LCell, rect::LRect}};

use crate::*;

const GRID_COLOR: Color = Color::rgba(0.75, 0.35, 0.25, 0.4);

const FACTOR: u16 = 4;
const LCELL_RADIUS: u16 = 30;
const HALF_COLS:u16 = 40;
const HALF_ROWS:u16 = 40;


#[derive(Resource, Deref, DerefMut, TypeUuid)]
#[uuid = "e458f087-eee5-48ee-bc11-f59f8826d4ae"]
pub struct Grid(pub DGrid);

impl Default for Grid {
    fn default() -> Self {
        
        Self(DGrid::new(FACTOR, LCELL_RADIUS, HALF_COLS, HALF_ROWS))
    }
}

#[derive(Component)]
pub struct LCol(pub u16);

#[derive(Component)]
pub struct LRow(pub u16);


#[derive(Bundle)]
pub struct GridBundle {
    pub col: LCol,
    pub row: LRow,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl GridBundle {

    pub fn new(grid:&DGrid, col:u16, row:u16) -> Self {

        let lcell = grid.loose.cells[row][col];

        if lcell.head == INVALID {

            Self {
                col: LCol(col),
                row: LRow(row),
    
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: GRID_COLOR.clone(),
                        anchor: Anchor::TopLeft,
                        ..default()
                        }, 
                    visibility: Visibility::Hidden,
                    ..default()
                }
            }

        }
        else {

            let lrect = lcell.rect;

            Self {
                col: LCol(col),
                row: LRow(row),
    
                sprite: SpriteBundle {
                    sprite: Sprite {
                        color: GRID_COLOR.clone(),
                        custom_size: Some( Vec2 {
                            x: lrect.w() as f32,
                            y: lrect.h() as f32
                        }),
                        anchor: Anchor::TopLeft,
                        ..default()
                        }, 
                    transform: Transform::from_translation( Vec3 {
                        x: lrect.l as f32,
                        y: lrect.t as f32,
                        z: 0.0
                    }),
                    visibility: Visibility::Visible,
                    ..default()
                }
            }

        }
    }
}

pub fn make_grids(
    mut commands: Commands,
    grid: Res<Grid>,
) {
    info!("make grid cells...");

    for row in 0..grid.loose.rows {
        for col in 0..grid.loose.cols {

            commands.spawn(GridBundle::new(&grid, col, row));
        }
    }

    commands.insert_resource(NextState(Some(GameState::Infoing)));
}


pub fn update_grids(
    mut query: Query<(
        &LCol,
        &LRow,
        &mut Sprite,
        &mut Transform,
        &mut Visibility,
    )>,
    grid: ResMut<Grid>,
) {

    let mut lcell:LCell;
    let mut lrect:LRect;

    for (col, row, mut sprite, mut transform, mut visibility) in query.iter_mut() {

        lcell = grid.loose.cells[row.0][col.0];

        if lcell.head == INVALID {
            *visibility = Visibility::Hidden;
        }
        else {
            *visibility = Visibility::Visible;

            lrect = lcell.rect;

            sprite.custom_size = Some( Vec2 {
                x: lrect.w() as f32,
                y: lrect.h() as f32
            });

            transform.translation.x = lrect.l as f32;
            transform.translation.y = lrect.t as f32;

        }
    }


}