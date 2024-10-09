use bevy::prelude::*;
use crate::component::{ButtonLabel, Bubble, SequenceDisplay};
use crate::constants::NORMAL_BUTTON;
use crate::constants::BACKGROUND_COLOR;
use crate::constants::BORDER_COLOR;


pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            background_color: BACKGROUND_COLOR.into(), // Set background to pink
            ..default()
        })
        .with_children(|parent| {
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
                    background_color: Color::rgb(0.3, 0.5, 0.9).into(), // Set bubble color to a lighter blue
                    ..default()
                },
                Bubble,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..default()
                    },
                    ..default()
                })
                .insert(SequenceDisplay);
            });

            let buttons: Vec<Vec<&str>> = vec![
                vec!["7", "8", "9", "/"],
                vec!["4", "5", "6", "*"],
                vec!["1", "2", "3", "-"],
                vec!["C", "0", "+/-", "+"],
                vec![".", "="],
            ];

            for row in buttons {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween, // Add space between buttons in a row
                            margin: UiRect {
                                bottom: Val::Px(10.0), // Add some margin between rows
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for label in row {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(65.0),
                                            margin: UiRect {
                                                left: Val::Px(5.0), // Add space between buttons in a row
                                                right: Val::Px(5.0), // Add space between buttons in a row
                                                top: Val::Px(10.0),
                                                bottom: Val::Px(10.0),
                                            },
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border: UiRect::all(Val::Px(5.0)), // Border size set here
                                            ..default()
                                        },
                                        background_color: NORMAL_BUTTON.into(),
                                        border_color: BorderColor(BORDER_COLOR), // Set the border color here
                                        ..default()
                                    },
                                    ButtonLabel(label.to_string()),
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            label.to_string(),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 32.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        ..default()
                                    });
                                });
                        }
                    });
            }
        });
}
