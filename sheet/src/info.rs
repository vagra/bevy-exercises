use bevy::{
    prelude::*,
    diagnostic::*
};

use common::assets::*;


const FONT_SIZE: f32 = 20.0;
const FONT_COLOR: Color = Color::YELLOW;

const INFO_TOP: f32 = 8.0;
const INFO_RIGHT: f32 = 8.0;


#[derive(Component, Clone)]
pub struct InfoText;


#[derive(Bundle)]
pub struct Info {
    pub info_text: InfoText,

    pub text_bundle: TextBundle,
}

impl Info {
    pub fn new(font_handle: &FontHandle) -> Self {

        let text_style = TextStyle {
            font: font_handle.0.clone(),
            font_size: FONT_SIZE,
            color: FONT_COLOR,
        };

        Self {
            info_text: InfoText,

            text_bundle: TextBundle::from_sections([
                TextSection::new("\nfps: ", text_style.clone()),    // 0
                TextSection::from_style(text_style.clone()),        // 1
    
                TextSection::new("\nnum: ", text_style.clone()),    // 2
                TextSection::from_style(text_style.clone()),        // 3
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(INFO_TOP),
                right: Val::Px(INFO_RIGHT),
                ..default()
            }),
        }

    }
}


pub fn make_info(
    mut commands: Commands,
    font_handle: Res<FontHandle>,
) {
    info!("make TextInfo");

    commands.spawn(Info::new(&font_handle));
}

pub fn update_info(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<InfoText>>
) {
    
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            let mut text = query.single_mut();
            text.sections[1].value =  format!("{value:.0}");
        }
    }

    if let Some(num) = diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
        if let Some(value) = num.value() {
            let mut text = query.single_mut();
            text.sections[3].value =  format!("{value:.0}");
        }
    }
}