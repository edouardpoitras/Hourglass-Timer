use bevy::prelude::*;
use crate::resources::{HourglassConfig, COLOR_PALETTE};
use crate::ui::LeftPanelMarker;

pub struct ColorPanelPlugin;

impl Plugin for ColorPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_color_buttons)
            .add_systems(Update, handle_color_button_clicks);
    }
}

#[derive(Component)]
struct ColorButton {
    color: Color,
}

fn spawn_color_buttons(
    mut commands: Commands,
    query: Query<Entity, With<LeftPanelMarker>>,
) {
    // Find the left panel container
    if let Ok(panel_entity) = query.single() {
        commands.entity(panel_entity).with_children(|parent| {
            // Add a label
            parent.spawn((
                Text::new("Colors"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Add color buttons
            for (i, &color) in COLOR_PALETTE.iter().enumerate() {
                parent.spawn((
                    Name::new(format!("Color Button {}", i)),
                    ColorButton { color },
                    Button,
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(color),
                    BorderColor(Color::WHITE),
                ));
            }
        });
    }
}

fn handle_color_button_clicks(
    mut interaction_query: Query<
        (&Interaction, &ColorButton, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut config: ResMut<HourglassConfig>,
) {
    for (interaction, color_button, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                config.color = color_button.color;
                *border_color = BorderColor(Color::srgb(0.0, 1.0, 0.0));
            }
            Interaction::Hovered => {
                *border_color = BorderColor(Color::srgb(0.8, 0.8, 0.8));
            }
            Interaction::None => {
                *border_color = BorderColor(Color::WHITE);
            }
        }
    }
}
