use std::collections::HashMap;
use bevy::prelude::*;

use crate::meta::*;


#[derive(Bundle, Clone)]
pub struct AnimatedSpriteSheetBundle {
    pub animation: Animation,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

#[derive(Component, Clone)]
pub struct Animation {
    pub clips: HashMap<String, Vec<ClipMeta>>,
    pub direction: usize,
    pub current_animation: Option<String>,
    pub current_index: usize,
    pub timer: Timer,
    pub once: bool,
}


impl Animation {

    pub fn new(sprite_sheet: &ActorSpriteSheetMeta) -> Self {

        let mut clips_map: HashMap<String, Vec<ClipMeta>> = HashMap::new();

        for (name, clip_meta) in sprite_sheet.animations.iter() {

            let mut clips_vec: Vec<ClipMeta> = Vec::new();

            for i in 0..8 {
                let mut clip_frames = clip_meta.frames.clone();

                for frame in clip_frames.iter_mut() {
                    *frame = *frame + sprite_sheet.columns * i;
                }

                let clip = ClipMeta{
                    name: clip_meta.name.clone(),
                    frames: clip_frames,
                    repeat: clip_meta.repeat,
                };

                clips_vec.push(clip);
            }

            clips_map.insert(name.to_string(), clips_vec);
        }

        Self {
            clips: clips_map,
            direction: 0,
            current_animation: None,
            current_index: 0,
            timer: Timer::from_seconds(sprite_sheet.fps, TimerMode::Once),
            once: false,
        }
    }

    pub fn play(&mut self, name: &str, direction: usize, repeating: bool) {

        self.direction = direction;
        self.current_animation = Some(name.to_owned());
        self.current_index = 0;
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
        &mut TextureAtlasSprite, 
    )>,
    time: Res<Time>,
) {
    for (mut animation, mut texture_atlas_sprite) in query.iter_mut() {

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
            texture_atlas_sprite.index = frame;
        }
    }
}