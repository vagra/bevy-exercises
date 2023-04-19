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

#[derive(Component)]
pub struct ID(pub u32);

impl ID {

    pub fn new(index:u32) -> Self {

        Self(index)
    }
}

#[derive(Component)]
pub struct Pos(pub Vec2);

impl Pos {
    pub fn new(x:f32, y:f32) -> Self {

        Self(Vec2::new(x, y))
    }
}

#[derive(Bundle)]
pub struct HeroBundle {
    pub id: ID,
    pub name: Name,
    pub prev_pos: Pos,
    pub move_action: MoveAction,

    #[bundle]
    pub animated_sprite_sheet_bundle: AnimatedSpriteSheetBundle,
}


impl HeroBundle {

    pub fn new(
        commands: &mut Commands,
        actor: &ActorMeta,
        entity: Entity,
        transform: &Transform,
    ) {
        let mut hero_bundle = HeroBundle {

            id: ID::new(entity.index()),

            prev_pos: Pos::new(
                        transform.translation.x,
                        transform.translation.y),

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
    mut actors: Query<(
            &Handle<ActorMeta>,
            Entity,
            &mut Transform,
        )>,
    actor_assets: Res<Assets<ActorMeta>>,
) {

    info!("make Heros with Grid...");

    let mut grid = Grid::default();

    for (actor_handle, entity, mut transform) in actors.iter_mut() {

        if let Some(actor) = actor_assets.get(actor_handle) {

            transform.translation.z = Z_MID - transform.translation.y * Z_SCALE;
            
            HeroBundle::new(
                &mut commands,
                actor,
                entity,
                &transform,
            );

            grid.insert(
                entity.index().clone(),
                transform.translation.x as i16,
                transform.translation.y as i16)
        }
    }

    info!("grid.pool.size: {}", grid.pool.size);

    commands.insert_resource(grid);
    commands.insert_resource(NextState(Some(GameState::Griding)));
}
