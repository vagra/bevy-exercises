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
    pub current_frame: usize,
    pub timer: Timer,
    pub once: bool,
}


impl Animation {

    pub fn new(sprite_sheet: &ActorSpriteSheetMeta) -> Self {

        let mut clips_map: HashMap<String, Vec<ClipMeta>> = HashMap::new();

        for (name, clip_meta) in sprite_sheet.animations.iter() {

            let mut clips_vec: Vec<ClipMeta> = Vec::new();

            for i in 0..8 {
                let clip = ClipMeta{
                    name: clip_meta.name.clone(),
                    start: sprite_sheet.columns * i + clip_meta.start,
                    end: sprite_sheet.columns * i + clip_meta.end,
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
            current_frame: 0,
            timer: Timer::from_seconds(sprite_sheet.fps, TimerMode::Once),
            once: false,
        }
    }

    pub fn play(&mut self, name: &str, direction: usize, repeating: bool) {

        self.direction = direction;
        self.current_animation = Some(name.to_owned());
        self.current_frame = 0;
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
        if let Some((_, end)) = self.get_current_indices() {
            if let Some(index) = self.get_current_index() {
                return index >= end;
            }
        }

        false
    }

    pub fn get_current_indices(&self) -> Option<(usize, usize)> {
        if let Some(animation) = &self.current_animation {
            match self.clips.get(animation) {
                Some(clips) => return Some((
                    clips[self.direction].start,
                    clips[self.direction].end,
                )),
                None => return None,
            }
        }

        None
    }

    pub fn get_current_index(&self) -> Option<usize> {
        if let Some((start, _)) = self.get_current_indices() {
            return Some(start + self.current_frame);
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
                    animation.current_frame = 0;
                }
            } else {
                animation.current_frame += 1;
            }
        }

        if let Some(index) = animation.get_current_index() {
            texture_atlas_sprite.index = index;
        }
    }
}