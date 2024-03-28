
use rand::Rng;
use bevy::prelude::*;

use crate::{
    *,
    animation::*,
};


#[derive(Component, Clone)]
pub struct Mover {
    pub direction: usize,
    pub animation: String,
    pub speed: f32,
    pub duration: f32,
    pub timer: Timer,
    pub pause: bool,
}

impl Mover {

    pub fn new() -> Self {

        let seconds = gen_rand_duration();
        let speed = gen_rand_speed();

        Self {
            direction: gen_rand_direction(),

            speed: speed,

            animation: gen_rand_animation(speed),

            duration: seconds,

            timer: Timer::from_seconds(seconds, TimerMode::Once),

            pause: false,
        }
    }

    pub fn random(&mut self) {
        self.direction = gen_rand_direction();
        self.speed = gen_rand_speed();
        self.animation = gen_rand_animation(self.speed);
        self.duration = gen_rand_duration();
        self.timer = Timer::from_seconds(self.duration, TimerMode::Once);
        self.pause = false;
    }

    pub fn back(&mut self, back: u8) {
        let mut rng = rand::thread_rng();
    
        let range: i32 = rng.gen_range(-1..2);
        self.direction = (back as i32 + range + DIRECTIONS as i32) as usize % DIRECTIONS;
    }

    pub fn _bump(&mut self) {
        let mut rng = rand::thread_rng();
    
        let range: i32 = rng.gen_range(-2..3);
        self.direction = (self.direction as i32 + range + DIRECTIONS as i32) as usize % DIRECTIONS;
    }

    pub fn dodge(&mut self, dirs:&Vec<usize>) -> bool {

        if dirs.contains(&self.direction) {
            return false;
        }

        let mut rng = rand::thread_rng();

        let index = rng.gen_range(0..dirs.len());
        self.direction = dirs[index];

        true
    }

    pub fn stop(&mut self) {

        self.pause = true;
    }

    pub fn play(& self, animation:&mut Animation) {

        animation.play(
            &self.animation, 
            &self.direction,
            true
        );
    }

}



fn gen_rand_direction() -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..DIRECTIONS)
}

fn gen_rand_speed() -> f32 {
    let mut rng = rand::thread_rng();

    let speed = rng.gen_range(0.0..MAX_SPEED);

    if speed < MIN_WALK_SPEED {
        return 0.0;
    }

    speed
}

fn gen_rand_animation(speed: f32) -> String {

    match speed {
        s if s < MIN_WALK_SPEED => "raise".to_string(),
        s if s >= MIN_RUN_SPEED => "run".to_string(),
        _ => "walk".to_string(),
    }
}

fn gen_rand_duration() -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(MIN_DURATION..MAX_DURATION)
}