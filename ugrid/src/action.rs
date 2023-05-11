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
        &Actor,
        &mut Mover,
        &mut Animation,
    )>,
    grid: Res<Grid>,
) {
    for (actor, mut mover, mut animation) in query.iter_mut() {

        if mover.speed < MIN_WALK_SPEED {
            continue;
        }

        if let Some(back_dir) = grid.out_bounds(
            actor.x as i16,
            actor.y as i16,
        ) {

            mover.back(back_dir);
            mover.play(&mut animation);
            continue;
        }

        let dirs = grid.query_dirs(
            actor.x as i16, actor.y as i16,
            actor.id
        );

        if dirs.len() > 0 {

            if mover.dodge(&dirs) {

                mover.play(&mut animation);
            }
        }
        else {

            mover.stop();
        }

        continue;
    }
}

pub fn moving(
    mut query: Query<(
        &mut Actor,
        &mut Mover,
        &mut Transform,
    )>,
    mut grid: ResMut<Grid>,
) {
    for (mut actor, mut mover, mut transform) in query.iter_mut() {

        if mover.speed < MIN_WALK_SPEED {
            continue;
        }

        if mover.pause {

            mover.pause = false;
            continue;
        }

        actor.prev_x = transform.translation.x;
        actor.prev_y = transform.translation.y;

        transform.translation.x += mover.speed * VECTORES[mover.dir].x;
        transform.translation.y += mover.speed * VECTORES[mover.dir].y;

        transform.translation.z = order_z(transform.translation.y);

        actor.x = transform.translation.x;
        actor.y = transform.translation.y;
        
        grid.move_cell(
            actor.id,
            actor.prev_x as i16, actor.prev_y as i16,
            actor.x as i16, actor.y as i16);
    }

}


pub fn random(
    mut query: Query<(
        &mut Mover,
        &mut Animation,
    )>,
    time: Res<Time>,
) {
    for (mut mover, mut animation) in query.iter_mut() {

        mover.timer.tick(time.delta());

        if mover.timer.finished() {
            mover.timer.reset();

            mover.random();

            mover.play(&mut animation);
        }
    }
}
