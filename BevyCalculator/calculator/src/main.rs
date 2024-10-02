use bevy::prelude::*;
use bevy::{color::palettes::basic::*, winit::WinitSettings};
use std::collections::VecDeque;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClickedButtons::default()) // Initialize with a default ClickedButtons resource
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
struct Bubble; // New component for the bubble

#[derive(Component)]
struct SequenceDisplay; // Component for the sequence display

#[derive(Resource, Default)]
struct ClickedButtons {
    buttons: Vec<String>, // Store both numbers and operators as strings
}

impl ClickedButtons {
    fn to_number_string(&self) -> String {
        self.buttons.join("") // Convert to a single string, retaining numbers and operators
    }

    fn to_number(&self) -> Option<String> {
        // Modify this method if needed based on how you want to handle operators
        Some(self.to_number_string())
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

                if button_label.0 == "=" {
                    // Compute the result of the current sequence
                    let result = evaluate_sequence(&clicked_buttons.to_number_string());
                    // Update the display to show the result
                    for mut text in text_query.iter_mut() {
                        text.sections[0].value = format!("{}", result);
                    }
                    // Clear the clicked buttons and set the result as the new starting point
                    clicked_buttons.buttons.clear();
                    clicked_buttons.buttons.push(result.clone()); // Push result into the ClickedButtons

                } else if button_label.0 == "C" {
                    // Clear the current sequence
                    clicked_buttons.buttons.clear();
                    // Update the display to show the cleared sequence
                    for mut text in text_query.iter_mut() {
                        text.sections[0].value = "".to_string();
                    }
                } else {
                    // Handle number and operator buttons
                    if let Ok(number) = button_label.0.parse::<u32>() {
                        clicked_buttons.buttons.push(number.to_string());
                    } else {
                        clicked_buttons.buttons.push(button_label.0.clone());
                    }

                    let number_string = clicked_buttons.to_number_string();
                    println!(
                        "Button {} clicked! Current sequence: {}",
                        button_label.0, number_string
                    );

                    // Update the text for the SequenceDisplay bubble
                    for mut text in text_query.iter_mut() {
                        text.sections[0].value = format!("{}", number_string);
                    }
                }
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



fn evaluate_sequence(sequence: &str) -> String {
    let mut numbers: VecDeque<f64> = VecDeque::new();  // Use a double-ended queue for efficient pop operations
    let mut operators: VecDeque<char> = VecDeque::new();
    let mut current_number = String::new();

    // Function to apply the operator to the top two numbers in the stack
    fn apply_operator(numbers: &mut VecDeque<f64>, operator: char) {
        let b = numbers.pop_back().unwrap();
        let a = numbers.pop_back().unwrap();
        let result = match operator {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => {
                if b == 0.0 {
                    panic!("Division by zero");
                }
                a / b
            }
            _ => panic!("Invalid operator"),
        };
        numbers.push_back(result);
    }

    for c in sequence.chars() {
        if c.is_digit(10) || c == '.' {
            // Check if the character is a digit or a decimal point
            current_number.push(c); // Build the current number
        } else if "+-*/()".contains(c) {
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    numbers.push_back(num);
                    current_number.clear(); // Clear the current number for the next one
                } else {
                    return "Invalid input".to_string(); // Handle case where number parsing fails
                }
            }

            // Handle parentheses and operator precedence
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
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<f64>() {
            numbers.push_back(num);
        } else {
            return "Invalid input".to_string();
        }
    }

    // Process remaining operators in the stack
    while let Some(operator) = operators.pop_back() {
        apply_operator(&mut numbers, operator);
    }

    numbers.pop_back().map_or("Invalid input".to_string(), |result| result.to_string())
}

// Helper function to determine the precedence of operators
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
    clicked_buttons: ResMut<ClickedButtons>,
) {
    // UI camera
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
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
                    background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
                    ..default()
                },
                Bubble,
            ))
            .with_children(|parent| {
                let number_string = clicked_buttons.to_number_string();
                parent.spawn((
                    TextBundle::from_section(
                        format!("{}", number_string), // Default text
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    ),
                    SequenceDisplay, // Add the new component here
                ));
            });

            // Container for the first row of buttons
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        width: Val::Percent(60.0),
                        margin: UiRect {
                            top: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, &asset_server, "1");
                    spawn_button(parent, &asset_server, "2");
                    spawn_button(parent, &asset_server, "3");
                });

            // Container for the second row of buttons
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        width: Val::Percent(60.0),
                        margin: UiRect {
                            top: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(parent, &asset_server, "4");
                    spawn_button(parent, &asset_server, "5");
                    spawn_button(parent, &asset_server, "6");
                });
            parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    width: Val::Percent(60.0),
                    margin: UiRect {
                        top: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                spawn_button(parent, &asset_server, "7");
                spawn_button(parent, &asset_server, "8");
                spawn_button(parent, &asset_server, "9");
            });
            parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    width: Val::Percent(60.0),
                    margin: UiRect {
                        top: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                spawn_button(parent, &asset_server, "0");
                spawn_button(parent, &asset_server, "+");
                spawn_button(parent, &asset_server, "-");
            });
            parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    width: Val::Percent(60.0),
                    margin: UiRect {
                        top: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                spawn_button(parent, &asset_server, "*");
                spawn_button(parent, &asset_server, "/");
                spawn_button(parent, &asset_server, ".");
            });
            parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    width: Val::Percent(60.0),
                    margin: UiRect {
                        top: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                spawn_button(parent, &asset_server, "(");
                spawn_button(parent, &asset_server, ")");
                spawn_button(parent, &asset_server, "=");
            });
            parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    // justify_content: JustifyContent::FlexStart,
                    justify_content: JustifyContent::SpaceEvenly,
                    width: Val::Percent(60.0),
                    margin: UiRect {
                        // left: Val::Px(160.0),
                        top: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                spawn_button(parent, &asset_server, "C");
            });
        });
}

fn spawn_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, label: &str) {
    parent.spawn(ButtonBundle {
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
    })
    .insert(ButtonLabel(label.to_string()))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            style: Style {
                // Ensure text itself is centered within the parent
                align_self: AlignSelf::Center,
                ..default()
            },
            text: Text::from_section(
                label,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        });
    });
}

