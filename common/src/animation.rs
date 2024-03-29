use std::collections::HashMap;
use rand::Rng;
use bevy::prelude::*;

use crate::{
    *,
    assets::*,
};


#[derive(Component, Clone)]
pub struct Animation {
    pub clips: HashMap<String, [ClipMeta; DIRECTIONS]>,
    pub direction: usize,
    pub current_animation: Option<String>,
    pub current_index: usize,
    pub timer: Timer,
    pub once: bool,
}


impl Animation {

    pub fn new(actor: &ActorAsset) -> Self {

        let mut clips_map: HashMap<String, [ClipMeta; DIRECTIONS]> = HashMap::new();

        for (name, clip_meta) in actor.animations.iter() {

            let mut clips_vec: [ClipMeta; DIRECTIONS] = [(); DIRECTIONS]
                .map(|_| ClipMeta::default());

            for i in 0..DIRECTIONS {
                let mut clip_frames = clip_meta.frames.clone();

                for frame in clip_frames.iter_mut() {
                    *frame = *frame + actor.columns * i;
                }

                clips_vec[i].name = clip_meta.name.clone();
                clips_vec[i].frames = clip_frames;
                clips_vec[i].repeat = clip_meta.repeat.clone();
            }

            clips_map.insert(name.to_string(), clips_vec);
        }

        Self {
            clips: clips_map,
            direction: 0,
            current_animation: None,
            current_index: 0,
            timer: Timer::from_seconds(actor.fps, TimerMode::Once),
            once: false,
        }
    }

    pub fn play(&mut self, name: &str, direction: &usize, repeating: bool) {

        self.direction = direction.clone();
        self.current_animation = Some(name.to_owned());
        self.current_index = self.get_random_index();
        self.timer.reset();
        self.timer.unpause();
        self.timer.set_mode(if repeating {
            TimerMode::Repeating
        } else {
            TimerMode::Once
        });
        self.once = false;
    }

    pub fn is_finished(&self) -> bool {
        self.once
    }

    pub fn is_repeating(&self) -> bool {
        if let Some(animation) = &self.current_animation {
            if let Some(clips) = self.clips.get(animation) {
                return clips[0].repeat;
            }
        }

        false
    }

    pub fn is_last_frame(&self) -> bool {
        if let Some(frames) = self.get_current_frames() {
            return self.current_index >= frames.len() - 1;
        }

        false
    }

    pub fn get_current_length(&self) -> Option<usize> {
        if let Some(frames) = &self.get_current_frames() {
            return Some(frames.len());
        }

        None
    }

    pub fn get_current_frames(&self) -> Option<&Vec<usize>> {
        if let Some(animation) = &self.current_animation {
            match self.clips.get(animation) {
                Some(clips) => return Some(
                    &clips[self.direction].frames
                ),
                None => return None,
            }
        }

        None
    }

    pub fn get_random_index(&self) -> usize {
        let mut rng = rand::thread_rng();

        return rng.gen_range(0..self.get_current_length().unwrap());
    }


    pub fn get_current_frame(&self) -> Option<usize> {
        if let Some(frames) = self.get_current_frames() {
            return Some(frames[self.current_index]);
        }

        None
    }
}


pub fn animating(
    mut query: Query<(
        &mut Animation,
        &mut TextureAtlas, 
    )>,
    time: Res<Time>,
) {
    for (mut animation, mut atlas) in query.iter_mut() {

        if animation.is_finished() && !animation.is_repeating() {
            continue;
        }

        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation.timer.reset();

            if animation.is_last_frame() {
                animation.once = true;

                if animation.is_repeating() {
                    animation.current_index = 0;
                }
            } else {
                animation.current_index += 1;
            }
        }

        if let Some(frame) = animation.get_current_frame() {
            atlas.index = frame;
        }
    }
}