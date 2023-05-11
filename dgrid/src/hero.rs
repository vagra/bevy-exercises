use bevy::{
    prelude::*,
    sprite::*,
};

use common::{
    *,
    bundle::*,
    animation::*,
    meta::*,
    mover::*,
};

use crate::{
    *,
    actor::*,
};

const MAX_SPAWN: u32 = 10000;
const ANCHOR: Vec2 = Vec2 {x: 0.0, y: -0.4};


#[derive(Bundle)]
pub struct HeroBundle {
    pub actor: Actor,
    pub mover: Mover,

    #[bundle]
    pub animated_sprite_sheet_bundle: AnimatedSpriteSheetBundle,
}


impl HeroBundle {

    pub fn new(index:u32, actor_meta: &ActorMeta) -> Self {

        let id = (actor_meta.id * MAX_SPAWN + index) as u32;

        let actor = Actor::new(index, id);
        let mover = Mover::new();

        let zoom = actor.hw as f32 / MAX_AGENT_RADIUS as f32 * SPRITE_SCALE;

        let transform = Transform{
            translation: Vec3 {
                x: actor.x,
                y: actor.y,
                z: order_z(actor.y)
            },
            scale: Vec3{
                x: zoom,
                y: zoom,
                z: 1.0
            },
            ..default()
        };

        Self {

            actor,
            mover,
            
            animated_sprite_sheet_bundle: AnimatedSpriteSheetBundle {

                sprite_sheet: SpriteSheetBundle {

                    sprite: TextureAtlasSprite {
                        
                        anchor: Anchor::Custom(ANCHOR),
                        ..default()
                    },

                    texture_atlas: actor_meta.sprite_sheet.atlas_handle.clone(),

                    transform,

                    ..default()
                },

                animation: Animation::new(

                    &actor_meta.sprite_sheet,
                ),
            },
        }
    }


    pub fn play(&mut self) {

        self
        .animated_sprite_sheet_bundle
        .animation
        .play(
            &self.mover.animation,
            self.mover.dir,
            true
        );
    }
}


pub fn make_heros(
    mut commands: Commands,
    query: Query<(
            &SpawnCount,
            &Handle<ActorMeta>,
        )>,
    actor_assets: Res<Assets<ActorMeta>>,
) {

    info!("make Heros with Grid...");

    let mut grid = Grid::default();

    for ( spawn_count, actor_handle) in query.iter() {

        if let Some(actor_meta) = actor_assets.get(actor_handle) {
            
            for index in 0..spawn_count.0 {

                let mut hero_bundle = HeroBundle::new(index, actor_meta,);
    
                grid.insert(
                    hero_bundle.actor.id,
                    hero_bundle.actor.x as i16,
                    hero_bundle.actor.y as i16,
                    hero_bundle.actor.hw,
                    hero_bundle.actor.hh
                );

                hero_bundle.play();

                commands.spawn(hero_bundle);

            }
        }
    }

    grid.optimize();

    info!("grid.loose.pool.size: {}", grid.loose.pool.size);

    commands.insert_resource(grid);
    commands.insert_resource(NextState(Some(GameState::Griding)));
}

