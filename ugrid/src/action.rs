use bevy::prelude::*;

use common::{
    *,
    animation::*,
    mover::*,
};

use crate::{
    actor::*,
    ugrid::*,
};

use grid::*;




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

        transform.translation.x += mover.speed * VECTORES[mover.direction].x;
        transform.translation.y += mover.speed * VECTORES[mover.direction].y;

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
