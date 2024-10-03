use bevy::prelude::*;
use bevy::{color::palettes::basic::*, winit::WinitSettings};
use std::collections::VecDeque;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClickedButtons::default())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const PURPLE: Color = Color::srgb(0.5, 0.0, 0.5);

#[derive(Component)]
struct ButtonLabel(String);

#[derive(Component)]
struct Bubble;

#[derive(Component)]
struct SequenceDisplay;

#[derive(Resource, Default)]
struct ClickedButtons {
    buttons: Vec<String>,
}

impl ClickedButtons {
    fn to_number_string(&self) -> String {
        self.buttons.join("") // Convert to a single string
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonLabel,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text, With<SequenceDisplay>>,
    mut clicked_buttons: ResMut<ClickedButtons>,
) {
    for (interaction, mut color, mut border_color, button_label) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = PURPLE;

                match button_label.0.as_str() {
                    "=" => {
                        // Compute the result of the current sequence
                        let result = evaluate_sequence(&clicked_buttons.to_number_string());
                        // Update the display to show the result
                        for mut text in text_query.iter_mut() {
                            text.sections[0].value = format!("{}", result);
                        }
                        // Clear the clicked buttons and set the result as the new starting point
                        clicked_buttons.buttons.clear();
                        clicked_buttons.buttons.push(result.clone()); // Push result into the ClickedButtons
                    }
                    "C" => {
                        // Clear the current sequence
                        clicked_buttons.buttons.clear();
                        // Update the display to show the cleared sequence
                        for mut text in text_query.iter_mut() {
                            text.sections[0].value = "".to_string();
                        }
                    }
                    "+/-" => {
                        // Toggle the sign of the last entered number
                        toggle_last_number_sign(&mut clicked_buttons);
                    }
                    _ => {
                        // Handle number and operator buttons
                        clicked_buttons.buttons.push(button_label.0.clone());
                    }
                }
                
                // Update the text for the SequenceDisplay bubble
                for mut text in text_query.iter_mut() {
                    text.sections[0].value = clicked_buttons.to_number_string(); // Update the display every time a button is pressed
                }

                println!(
                    "Button {} clicked! Current sequence: {}",
                    button_label.0, clicked_buttons.to_number_string()
                );
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn toggle_last_number_sign(clicked_buttons: &mut ClickedButtons) {
    if let Some(last) = clicked_buttons.buttons.last_mut() {
        if let Ok(mut number) = last.parse::<f64>() {
            number = -number;
            *last = number.to_string(); // Update the last button to reflect the new sign
        } else if last == "+" {
            *last = "-".to_string(); // Toggle between "+" and "-" if the last entry is a sign
        } else if last == "-" {
            *last = "+".to_string();
        }
    }
}


fn evaluate_sequence(sequence: &str) -> String {
    let mut numbers: VecDeque<f64> = VecDeque::new();
    let mut operators: VecDeque<char> = VecDeque::new();
    let mut current_number = String::new();

    fn apply_operator(numbers: &mut VecDeque<f64>, operator: char) {
        let b = numbers.pop_back().unwrap();
        let a = numbers.pop_back().unwrap();
        let result = match operator {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => {
                if b == 0.0 {
                    println!("Invalid input: Division by zero");
                    return; // Exit the function if division by zero
                    }
                a / b
            }
            _ =>      {println!("Invalid input: Invalid operator");
            return;} // Exit the function if invalid operator

        };
        numbers.push_back(result);
    }

    let mut prev_char: Option<char> = None;

    for c in sequence.chars() {
        if c.is_digit(10) || c == '.' {
            current_number.push(c);
        } else if c == '-' && (prev_char.is_none() || "+-*/(".contains(prev_char.unwrap())) {
            // Handle negation as a standalone number
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    numbers.push_back(num);
                    current_number.clear();
                }
            }
            current_number.push(c); // Add the '-' to current number
        } else if "+-*/()".contains(c) {
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    numbers.push_back(num);
                    current_number.clear();
                } else {
                    return "Invalid input".to_string();
                }
            }

            if c == '(' {
                operators.push_back(c);
            } else if c == ')' {
                while let Some(&top_operator) = operators.back() {
                    if top_operator == '(' {
                        operators.pop_back();
                        break;
                    }
                    apply_operator(&mut numbers, operators.pop_back().unwrap());
                }
            } else {
                while let Some(&top_operator) = operators.back() {
                    if precedence(top_operator) >= precedence(c) {
                        apply_operator(&mut numbers, operators.pop_back().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push_back(c);
            }
        }
        prev_char = Some(c);
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<f64>() {
            numbers.push_back(num);
        } else {
            return "Invalid input".to_string();
        }
    }


    while let Some(operator) = operators.pop_back() {
        apply_operator(&mut numbers, operator);
    }

    // Round the final result
    let final_result = numbers.pop_back().map_or("Invalid input".to_string(), |result| {
        format!("{:.4}", result) // Round to 2 decimal places
    });

    final_result}


fn precedence(operator: char) -> usize {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn setup(
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
