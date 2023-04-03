use rand::Rng;
use bevy::prelude::*;

use crate::{
    actor::*,
    animation::*,
};

pub const DIRECTIONS: usize = 8;

const MIN_DURATION: f32 = 2.0;
const MAX_DURATION: f32 = 8.0;

const Z_MID: f32 = 100.0;
const Z_SCALE: f32 = 0.01;

const MAX_SPEED: f32 = 2.0;
const MIN_RUN_SPEED: f32 = 1.0;
const MIN_WALK_SPEED: f32 = 0.5;

const SQR: f32 = 0.7071;

const VECTORES: [Vec2; 8] = [
	Vec2{ x: 0.0, y:-1.0 },
	Vec2{ x: SQR, y:-SQR },
	Vec2{ x: 1.0, y: 0.0 },
	Vec2{ x: SQR, y: SQR },
	Vec2{ x: 0.0, y: 1.0 },
	Vec2{ x:-SQR, y: SQR },
	Vec2{ x:-1.0, y: 0.0 },
	Vec2{ x:-SQR, y:-SQR },
];


#[derive(Component, Clone)]
pub struct MoveAction {
    pub direction: usize,
    pub animation: String,
    pub speed: f32,
    pub duration: f32,
    pub timer: Timer,
}

impl MoveAction {

    pub fn new() -> Self {

        let seconds = gen_random_duration();
        let speed = gen_random_speed();

        Self {
            direction: gen_random_direction(),

            speed: speed,

            animation: gen_random_animation(speed),

            duration: seconds,

            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }

    pub fn random(&mut self) {
        self.direction = gen_random_direction();
        self.speed = gen_random_speed();
        self.animation = gen_random_animation(self.speed);
        self.duration = gen_random_duration();
        self.timer = Timer::from_seconds(self.duration, TimerMode::Once);
    }

    fn back(&mut self, back: usize) {
        let mut rng = rand::thread_rng();
    
        let range: i32 = rng.gen_range(-1..2);
        self.direction = (back as i32 + range + DIRECTIONS as i32) as usize % DIRECTIONS;
    }

}

pub fn backing(
    mut query: Query<(
        &mut Transform,
        &mut MoveAction,
        &mut Animation,
    )>
) {
    for (transform, mut action, mut animation) in query.iter_mut() {
        if let Some(back_direction) = check_region(
            transform.translation.x,
            transform.translation.y,
        ) {
            action.back(back_direction);

            animation.play(
                action.animation.as_str(), 
                &action.direction, 
                true
            );
        }
    }
}

pub fn moving(
    mut query: Query<(
        &mut Transform,
        &mut MoveAction,
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
        &mut MoveAction,
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


pub fn check_region(x: f32, y: f32) -> Option<usize> {
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


fn gen_random_direction() -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..DIRECTIONS)
}

fn gen_random_speed() -> f32 {
    let mut rng = rand::thread_rng();

    let speed = rng.gen_range(0.0..MAX_SPEED);

    if speed < MIN_WALK_SPEED {
        return 0.0;
    }

    speed
}

fn gen_random_animation(speed: f32) -> String {

    match speed {
        s if s < MIN_WALK_SPEED => "raise".to_string(),
        s if s >= MIN_RUN_SPEED => "run".to_string(),
        _ => "walk".to_string(),
    }
}

fn gen_random_duration() -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(MIN_DURATION..MAX_DURATION)
}