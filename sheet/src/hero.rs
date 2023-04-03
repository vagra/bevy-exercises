use bevy::{
    prelude::*,
    sprite::*,
};

use crate::{
    *,
    action::*,
    animation::*,
    meta::*,
};


const ANCHOR: Vec2 = Vec2 {x: 0.0, y: -0.4};


#[derive(Bundle)]
pub struct HeroBundle {
    pub name: Name,
    pub move_action: MoveAction,

    #[bundle]
    pub animated_sprite_sheet_bundle: AnimatedSpriteSheetBundle,
}


impl HeroBundle {

    pub fn instantiate(
        commands: &mut Commands,
        actor: &ActorMeta,
        entity: Entity,
        transform: &Transform,
    ) {
        let mut hero_bundle = HeroBundle {

            name: Name::new(actor.name.clone()),

            move_action: MoveAction::new(),
            
            animated_sprite_sheet_bundle: AnimatedSpriteSheetBundle {

                sprite_sheet: SpriteSheetBundle {

                    sprite: TextureAtlasSprite {
                        
                        anchor: Anchor::Custom(ANCHOR),
                        ..default()
                    },

                    texture_atlas: actor.sprite_sheet.atlas_handle.clone(),

                    transform: *transform,

                    ..Default::default()
                },

                animation: Animation::new(
                    &actor.sprite_sheet,
                ),
            },
        };

        hero_bundle
            .animated_sprite_sheet_bundle
            .animation
            .play(
                &hero_bundle.move_action.animation,
                &hero_bundle.move_action.direction,
                true);

        commands
            .entity(entity)
            .insert(hero_bundle);

    }
}


pub fn make_heros(
    mut commands: Commands,
    actors: Query<(
            &Handle<ActorMeta>,
            Entity,
            &Transform,
        )>,
    actor_assets: Res<Assets<ActorMeta>>,
) {

    info!("make Heros...");

    for (actor_handle, entity, transform, ) in actors.iter() {

        if let Some(actor) = actor_assets.get(actor_handle) {
            
            HeroBundle::instantiate(
                &mut commands,
                actor,
                entity,
                transform,
            );
        }
    }

    commands.insert_resource(NextState(Some(GameState::Playing)));
}