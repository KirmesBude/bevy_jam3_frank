use bevy::prelude::*;

use crate::{player::Player, stats::base::Health};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(update_health_ui);
    }
}

#[derive(Debug, Default, Component)]
struct PlayerHealthText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Health: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::RED,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 60.0,
                color: Color::BLACK,
            }),
        ]),
        PlayerHealthText,
    ));
}

fn update_health_ui(
    player_health: Query<&Health, With<Player>>,
    mut query: Query<&mut Text, With<PlayerHealthText>>,
) {
    if let Ok(health) = player_health.get_single() {
        for mut text in &mut query {
            let value = health.0;
            text.sections[1].value = format!("{value:.1}");
        }
    }
}
