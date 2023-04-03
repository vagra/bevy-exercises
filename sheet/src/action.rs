use rand::Rng;
use bevy::prelude::*;

use crate::{
    animation::*,
};

pub const DIRECTIONS: usize = 8;

const MIN_DURATION: f32 = 2.0;
const MAX_DURATION: f32 = 8.0;


#[derive(Component, Clone)]
pub struct MoveAction {
    pub direction: usize,
    pub animation: String,
    pub duration: f32,
    pub timer: Timer,
}

impl MoveAction {

    pub fn new() -> Self {

        let seconds = get_random_duration();

        Self {
            direction: get_random_direction(),

            animation: get_random_animation(),

            duration: seconds,

            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }

    }

}


pub fn moving(
    mut query: Query<(
        &mut MoveAction,
        &mut Animation,
    )>,
    time: Res<Time>,
) {
    for (mut move_action, mut animation) in query.iter_mut() {

        move_action.timer.tick(time.delta());

        if move_action.timer.finished() {
            move_action.timer.reset();

            move_action.direction = get_random_direction();
            move_action.animation = get_random_animation();

            animation.play(
                move_action.animation.as_str(), 
                move_action.direction, 
                true
            );
        }
    }
}

pub fn get_random_direction() -> usize {
    let mut rng = rand::thread_rng();

    return rng.gen_range(0..DIRECTIONS);
}

pub fn get_random_animation() -> String {
    let mut rng = rand::thread_rng();

    match rng.gen_range(0..ANIMATIONS) {
        0 => "run".to_string(),
        1 => "walk".to_string(),
        _ => "raise".to_string(),
    }
}

pub fn get_random_duration() -> f32 {
    let mut rng = rand::thread_rng();

    return rng.gen_range(MIN_DURATION..MAX_DURATION);
}