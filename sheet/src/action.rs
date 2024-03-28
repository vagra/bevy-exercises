use bevy::prelude::*;

use crate::hero::*;
use common::{
    *,
    animation::*,
    mover::*,
};


pub fn backing(
    mut query: Query<(
        &mut Transform,
        &mut Mover,
        &mut Animation,
    )>
) {
    
    for (transform, mut mover, mut animation) in query.iter_mut() {
        if let Some(back_direction) = check_region(
            transform.translation.x,
            transform.translation.y,
        ) {
            mover.back(back_direction);

            animation.play(
                mover.animation.as_str(), 
                &mover.direction, 
                true
            );
        }
    }
}

pub fn moving(
    mut query: Query<(
        &mut Transform,
        &mut Mover,
    )>
) {
    
    for (mut transform, action) in query.iter_mut() {
        transform.translation.x += action.speed * VECTORES[action.direction].x;
        transform.translation.y += action.speed * VECTORES[action.direction].y;

        // z-order
        transform.translation.z = Z_MID - transform.translation.y * Z_SCALE;
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
                &move_action.direction, 
                true
            );
        }
    }
}


pub fn check_region(x: f32, y: f32) -> Option<u8> {
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

