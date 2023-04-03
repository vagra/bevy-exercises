use bevy::prelude::*;
use bevy::diagnostic::*;


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
    pub fn new(font_handle: &FontHandle) -> Self {

        let text_style = TextStyle {
            font: font_handle.0.clone(),
            font_size: 20.0,
            color: Color::YELLOW,
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
                position: UiRect {
                    top: Val::Px(8.0),
                    right: Val::Px(8.0),
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
) {
    commands.spawn(Info::new(&font_handle));
}

pub fn update_info(
    diagnostics: Res<Diagnostics>,
    mut info_text: Query<&mut Text, With<InfoText>>
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            let mut text = info_text.single_mut();
            text.sections[1].value =  format!("{value:.0}");
        }
    }

    if let Some(num) = diagnostics.get(EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
        if let Some(value) = num.value() {
            let mut text = info_text.single_mut();
            text.sections[3].value =  format!("{value:.0}");
        }
    }
}