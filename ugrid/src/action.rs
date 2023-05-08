use bevy::prelude::*;

use crate::{
    actor::*,
    animation::*,
    hero::*,
    mover::*,
    ugrid::*,
};

use grid::*;


pub const DIRECTIONS: usize = 8;

pub const MIN_DURATION: f32 = 2.0;
pub const MAX_DURATION: f32 = 8.0;

pub const Z_MID: f32 = 100.0;
pub const Z_SCALE: f32 = 0.01;

pub const MAX_SPEED: f32 = 1.2;
pub const MIN_RUN_SPEED: f32 = 0.8;
pub const MIN_WALK_SPEED: f32 = 0.2;

const SQR: f32 = 0.7071;

pub const VECTORES: [Vec2; 8] = [
	Vec2{ x: 0.0, y:-1.0 },
	Vec2{ x: SQR, y:-SQR },
	Vec2{ x: 1.0, y: 0.0 },
	Vec2{ x: SQR, y: SQR },
	Vec2{ x: 0.0, y: 1.0 },
	Vec2{ x:-SQR, y: SQR },
	Vec2{ x:-1.0, y: 0.0 },
	Vec2{ x:-SQR, y:-SQR },
];



pub fn turning(
    mut query: Query<(
        &ID,
        &Transform,
        &mut Mover,
        &mut Animation,
    )>,
    grid: Res<Grid>,
) {
    for (id, transform, mut action, mut animation) in query.iter_mut() {

        if action.speed < MIN_WALK_SPEED {
            continue;
        }

        if let Some(back_dir) = grid.out_bounds(
            transform.translation.x as i16,
            transform.translation.y as i16,
        ) {
            action.back(back_dir);

            animation.play(
                action.animation.as_str(), 
                &action.dir, 
                true
            );

            continue;
        }

        let dirs = grid.query_dirs(
            transform.translation.x as i16, transform.translation.y as i16, id.0
        );

        if dirs.len() > 0 {

            if action.dodge(&dirs) {

                animation.play(
                    action.animation.as_str(), 
                    &action.dir, 
                    true
                );
            }
        }
        else {

            action.stop();
        }

        continue;
    }
}

pub fn moving(
    mut query: Query<(
        &ID,
        &mut Pos,
        &mut Transform,
        &mut Mover,
    )>,
    mut grid: ResMut<Grid>,
) {
    for (id, mut prev_pos, mut transform, mut action) in query.iter_mut() {

        if action.speed < MIN_WALK_SPEED {
            continue;
        }

        if action.pause {

            action.pause = false;
            continue;
        }

        prev_pos.0.x = transform.translation.x;
        prev_pos.0.y = transform.translation.y;

        transform.translation.x += action.speed * VECTORES[action.dir].x;
        transform.translation.y += action.speed * VECTORES[action.dir].y;

        // z-order
        transform.translation.z = Z_MID - transform.translation.y * Z_SCALE;
        
        grid.move_cell(id.0, prev_pos.0.x as i16, prev_pos.0.y as i16,
            transform.translation.x as i16, transform.translation.y as i16);
    }

}


pub fn random(
    mut query: Query<(
        &mut Mover,
        &mut Animation,
    )>,
    time: Res<Time>,
) {
    for (mut move_action, mut animation) in query.iter_mut() {

        move_action.timer.tick(time.delta());

        if move_action.timer.finished() {
            move_action.timer.reset();

            move_action.random();

            animation.play(
                move_action.animation.as_str(), 
                &move_action.dir, 
                true
            );
        }
    }
}


pub fn _check_region(x: f32, y: f32) -> Option<usize> {
    let l = REGION.min.x;
    let b = REGION.min.y;
    let r = REGION.max.x;
    let t = REGION.max.y;

    if x < l && y > t {return Some(1);}
    if x < l && y < b {return Some(3);}
    if x > r && y < b {return Some(5);}
    if x > r && y > t {return Some(7);}

    if y > t {return Some(0);}
    if x < l {return Some(2);}
    if y < b {return Some(4);}
    if x > r {return Some(6);}

    return None;

}

