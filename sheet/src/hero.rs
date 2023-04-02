use rand::Rng;

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
                        
                        anchor: Anchor::Custom(
                            Vec2::new(0.0, -0.4)
                        ),
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

        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0.0..8.0) as usize;

        hero_bundle
            .animated_sprite_sheet_bundle
            .animation
            .play("run", direction, true);

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