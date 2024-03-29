use bevy::{
    prelude::*,
    sprite::*,
};

use common::{
    *,
    animation::*,
    assets::*,
    mover::*
};

use crate::{
    *,
    actor::*,
};

const MAX_SPAWN: u32 = 10000;
const SCALE: Vec3 = Vec3 { x: 0.5, y: 0.5, z: 1.0 };
const ANCHOR: Vec2 = Vec2 {x: 0.0, y: -0.4};
const SPAWN_NUM: u32 = 200;


#[derive(Bundle)]
pub struct HeroBundle {
    pub actor: Actor,
    pub mover: Mover,

    pub animation: Animation,
    pub sprite_sheet: SpriteSheetBundle,
}


impl HeroBundle {

    pub fn new(
        index: u32,
        asset: &ActorAsset
    ) -> Self {

        let id = (asset.id * MAX_SPAWN + index) as u32;

        let actor = Actor::new(index, id);
        let mover = Mover::new();

        let position = Vec3 {
            x: actor.x, 
            y: actor.y,
            z: order_z(actor.y),
        };

        let transform = Transform{
            translation: position,
            scale: SCALE,
            ..default()
        };

        Self {

            actor,
            mover,

            animation: Animation::new(

                &asset,
            ),

            sprite_sheet: SpriteSheetBundle {

                sprite: Sprite {
                    
                    anchor: Anchor::Custom(ANCHOR),
                    ..default()
                },

                atlas: TextureAtlas {
                    layout: asset.layout_handle.clone(),
                    index: 0
                },

                texture: asset.image_handle.clone(),

                transform,

                ..default()
            },
        }

    }

    pub fn spawn(
        commands: &mut Commands,
        grid: &mut Grid,
        index: u32,
        actor: &ActorAsset
    ) {

        let mut hero = HeroBundle::new(index, actor);

        hero.play();

        grid.insert(
            hero.actor.id,
            hero.actor.x as i16,
            hero.actor.y as i16,
        );

        commands.spawn(hero);

    }


    pub fn play(&mut self) {

        self.animation.play(
            &self.mover.animation,
            &self.mover.direction,
            true
        );
    }
}


pub fn make_heros(
    mut commands: Commands,
    actor_handles: Res<ActorHandles>,
    actor_assets: Res<Assets<ActorAsset>>,
) {

    info!("make Heros with Grid...");

    let mut grid = Grid::default();

    for actor_handle in actor_handles.0.iter() {

        if let Some(actor) = actor_assets.get(&actor_handle.0) {
            
            info!("spawn actor.name: {}", actor.name);

            for i in 0u32..SPAWN_NUM {
                HeroBundle::spawn(&mut commands, &mut grid, i, &actor);
            }
        }
    }

    info!("grid.pool.size: {}", grid.pool.size);

    commands.insert_resource(grid);
    commands.insert_resource(NextState(Some(GameState::Griding)));
}