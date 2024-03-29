use bevy::prelude::*;

pub mod animation;
pub mod assets;
pub mod camera;
pub mod mover;



pub const DIRECTIONS: usize = 8;

pub const MIN_DURATION: f32 = 2.0;
pub const MAX_DURATION: f32 = 8.0;

pub const Z_MID: f32 = 100.0;
pub const Z_SCALE: f32 = 0.01;

pub const MAX_SPEED: f32 = 1.2;
pub const MIN_RUN_SPEED: f32 = 0.8;
pub const MIN_WALK_SPEED: f32 = 0.2;

const ACTOR_NUM: i32 = 17;
const SQR: f32 = 0.7071;
const FONT_TTF: &str = "fonts/FiraCode-Regular.ttf";

pub const VECTORES: [Vec2; 8] = [
	Vec2{ x: 0.0, y:-1.0 },
	Vec2{ x: SQR, y:-SQR },
	Vec2{ x: 1.0, y: 0.0 },
	Vec2{ x: SQR, y: SQR },
	Vec2{ x: 0.0, y: 1.0 },
	Vec2{ x:-SQR, y: SQR },
	Vec2{ x:-1.0, y: 0.0 },
	Vec2{ x:-SQR, y:-SQR },
];

pub const BASE_PATH: &str = "sheet";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Griding,
    Infoing,
    Playing,
    Paused,
}



pub fn order_z(y:f32) -> f32 {
    Z_MID - y * Z_SCALE
}
