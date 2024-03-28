use bevy::{
    prelude::*,
    sprite::*,
};
use rand::Rng;

use crate::{
    *,
    action::*,
    animation::*,
};


pub const REGION: Rect = Rect{
    min: Vec2 { x: -800.0, y: -400.0 },
    max: Vec2 { x: 800.0, y: 400.0 },
};
const SCALE: Vec3 = Vec3 { x: 0.8, y: 0.8, z: 1.0 };
const ANCHOR: Vec2 = Vec2 {x: 0.0, y: -0.4};


#[derive(Bundle)]
pub struct HeroBundle {
    pub name: Name,
    pub move_action: MoveAction,
    pub animation: Animation,
    pub sprite_sheet: SpriteSheetBundle,
}


impl HeroBundle {

    pub fn instantiate(
        commands: &mut Commands,
        actor: &ActorAsset,
    ) {

        let mut rng = rand::thread_rng();

        let position = Vec3 {
            x: rng.gen_range(REGION.min.x..REGION.max.x), 
            y: rng.gen_range(REGION.min.y..REGION.max.y),
            z: 0.0,
        };

        let transform = Transform {
            translation: position,
            scale: SCALE,
            ..default()
        };

        let mut hero_bundle = HeroBundle {

            name: Name::new(actor.name.clone()),

            move_action: MoveAction::new(),
            
            animation: Animation::new(
                &actor,
            ),

            sprite_sheet: SpriteSheetBundle {

                sprite: Sprite {
                    anchor: Anchor::Custom(ANCHOR),
                    ..default()
                },

                atlas: TextureAtlas {
                    layout: actor.layout_handle.clone(),
                    index: 0
                },

                texture: actor.image_handle.clone(),

                transform: transform,

                ..Default::default()
            },

        };

        hero_bundle.animation.play(
            &hero_bundle.move_action.animation,
            &hero_bundle.move_action.direction,
            true
        );

        commands.spawn(hero_bundle);

    }
}


pub fn make_heros(
    mut commands: Commands,
    actor_handles: Res<ActorHandles>,
    actor_assets: Res<Assets<ActorAsset>>,
) {

    info!("make Heros...");

    for actor_handle in actor_handles.0.iter() {
        if let Some(actor) = actor_assets.get(&actor_handle.0) {
            info!("spawn actor.name: {}", actor.name);

            for _i in 0..SPAWN_NUM {
                HeroBundle::instantiate(&mut commands, &actor);
            }
        }
    }

    info!("heros spawned. playing...");
    commands.insert_resource(NextState(Some(GameState::Playing)));
}

    