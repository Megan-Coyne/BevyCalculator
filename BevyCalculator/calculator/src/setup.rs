use bevy::prelude::*;
use crate::component::{ButtonLabel, Bubble, SequenceDisplay};
use crate::constants::NORMAL_BUTTON;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Bubble for displaying the current sequence
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        height: Val::Px(50.0),
                        margin: UiRect {
                            top: Val::Px(20.0),
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..default()
                },
                Bubble,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(), // Initialize as empty
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..default()
                    },
                    ..default()
                }).insert(SequenceDisplay); // Attach the SequenceDisplay component here
            });

            let buttons: Vec<Vec<&str>> = vec![
                vec!["7", "8", "9", "/"],
                vec!["4", "5", "6", "*"],
                vec!["1", "2", "3", "-"],
                vec!["C", "0", "+/-", "+"],
                vec![".", "="],
            ];

            for row in buttons {
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for label in row {
                        parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(150.0),
                                    height: Val::Px(65.0),
                                    justify_content: JustifyContent::Center, // Center child items horizontally
                                    align_items: AlignItems::Center,         // Center child items vertically
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            ButtonLabel(label.to_string()),
                        ))
                        .insert(Interaction::None)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                label.to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    }
                });
            }
        });
}
