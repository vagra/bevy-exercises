#![allow(dead_code)]

use std::{time::Duration, f32::consts::PI};

use bevy::prelude::*;

const FULL_ANGLES: i32 = 360;
const HALF_ANGLES: i32 = 180;
const DIR_ANGLES: i32 = 45;
const REDIAN_ANGLES: i32 = 180;
const ANGLE2REDIAN: f32 = PI / 180.0;
const REDIAN_ANGLE: f32 = PI / 180.0;
const TURN_SPEED: i32 = 15;
const RUN_SPEED: f32 = 0.1;


#[derive(PartialEq)]
pub enum Action {
    Run,
    Stand,
    Attack,
}


#[derive(Resource)]
pub struct Animations(
    pub Vec<Handle<AnimationClip>>
);


#[derive(Component)]
pub struct Actor {
    pub action: Action,
    pub position: Vec3,
    pub curr_angle: i32,
    pub dest_angle: i32,
}



impl Default for Actor {
    fn default() -> Self {
        Actor {
            action: Action::Stand,
            position: Vec3::ZERO,
            curr_angle: 0,
            dest_angle: 0,
        }
    }
}


impl Actor {

    pub fn stand(&mut self, player: &mut AnimationPlayer, animations: &Animations) {
        self.play(1, player, animations);

        if self.action != Action::Stand {
            self.action = Action::Stand;
        }
    }

    pub fn run(&mut self, player: &mut AnimationPlayer, animations: &Animations) {
        if self.action != Action::Run {
            self.play(3, player, animations);
            self.action = Action::Run;
        }
    }

    pub fn curr_redian(&self) -> f32 {

        return self.curr_angle as f32 * ANGLE2REDIAN;
    }

    pub fn dest_redian(&self) -> f32 {

        return self.dest_angle as f32 * ANGLE2REDIAN;
    }

    pub fn set_dir(&mut self, dir: usize) {
        self.dest_angle = dir as i32 * DIR_ANGLES;
    }

    pub fn turning(&mut self) {
        if self.curr_angle == self.dest_angle {
            return;
        }

        let mut diff_angle = self.dest_angle - self.curr_angle;
        if diff_angle < - HALF_ANGLES {
            diff_angle += FULL_ANGLES;
        }
        else if diff_angle > HALF_ANGLES {
            diff_angle -= FULL_ANGLES;
        }

        let step_angle = if diff_angle > 0 {
                            TURN_SPEED
                        } else {
                            - TURN_SPEED
                        };

        self.curr_angle = (self.curr_angle + step_angle + FULL_ANGLES) % FULL_ANGLES;
    }

    pub fn moving(&mut self) {

        if self.action == Action::Run {

            let mut offset: Vec3 = Vec3::new(0.0, 0.0, 1.0) * RUN_SPEED;
            let rotation: Quat = Quat::from_rotation_y(self.dest_redian());
            offset = rotation * offset;

            self.position += offset;
        }
    }

    fn play(&self, index: usize, player: &mut AnimationPlayer, animations: &Animations) {
        player
            .play_with_transition(
                animations.0[index].clone_weak(),
                Duration::from_millis(100),
            )
            .set_speed(0.5)
            .repeat();
    }
}