#![allow(dead_code)]

use std::time::Duration;

use bevy::prelude::*;


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
}



impl Default for Actor {
    fn default() -> Self {
        Actor {
            action: Action::Stand,
        }
    }
}


impl Actor {

    pub fn stand(&self, player: &mut AnimationPlayer, animations: &Animations) {
        self.play(1, player, animations);
    }

    pub fn run(&self, player: &mut AnimationPlayer, animations: &Animations) {
        if self.action != Action::Run {
            self.play(3, player, animations);
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