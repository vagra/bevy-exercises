use bevy::prelude::*;
use bevy::diagnostic::*;

use crate::GameState;
use crate::ugrid::Grid;


const FONT_SIZE: f32 = 20.0;
const FONT_COLOR: Color = Color::YELLOW;

const INFO_TOP: f32 = 8.0;
const INFO_RIGHT: f32 = 8.0;


#[derive(Resource, Deref, DerefMut)]
pub struct FontHandle(pub Handle<Font>);


#[derive(Component, Clone)]
pub struct InfoText;


#[derive(Bundle, Clone)]
pub struct Info {
    pub info_text: InfoText,

    #[bundle]
    pub text_bundle: TextBundle,
}

impl Info {
    pub fn new(font_handle: &FontHandle, agents:u16) -> Self {

        let text_style = TextStyle {
            font: font_handle.0.clone(),
            font_size: FONT_SIZE,
            color: FONT_COLOR,
        };

        let agents = format!("\nagents: {}", agents);

        Self {
            info_text: InfoText,

            text_bundle: TextBundle::from_sections([
                TextSection::new("\nfps: ", text_style.clone()),    // 0
                TextSection::from_style(text_style.clone()),        // 1
    
                TextSection::new(agents, text_style.clone()),    // 2
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(INFO_TOP),
                    right: Val::Px(INFO_RIGHT),
                    ..default()
                },
                ..default()
            }),
        }

    }
}


pub fn make_info(
    mut commands: Commands,
    font_handle: Res<FontHandle>,
    grid: Res<Grid>
) {
    info!("make TextInfo");

    commands.spawn(Info::new(&font_handle, grid.pool.size));

    commands.insert_resource(NextState(Some(GameState::Playing)));

    info!("playing...")
}

pub fn update_info(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<InfoText>>
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            let mut text = query.single_mut();
            text.sections[1].value =  format!("{value:.0}");
        }
    }
}