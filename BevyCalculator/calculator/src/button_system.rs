use bevy::prelude::*;
use crate::component::{ButtonLabel, SequenceDisplay};
use crate::constants::{NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON, PURPLE, BORDER_COLOR};
use crate::clicked_buttons::ClickedButtons;
use crate::evaluate::{evaluate_sequence, toggle_last_number_sign};

pub fn button_system(
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
                *color = PRESSED_BUTTON.into(); // Darker color on press
                border_color.0 = BORDER_COLOR; // Maintain border color on press

                match button_label.0.as_str() {
                    "=" => {
                        let result = evaluate_sequence(&clicked_buttons.to_number_string());
                        for mut text in text_query.iter_mut() {
                            text.sections[0].value = format!("{}", result);
                        }
                        clicked_buttons.buttons.clear();
                        clicked_buttons.buttons.push(result.clone());
                    }
                    "C" => {
                        clicked_buttons.buttons.clear();
                        for mut text in text_query.iter_mut() {
                            text.sections[0].value = "".to_string();
                        }
                    }
                    "+/-" => toggle_last_number_sign(&mut clicked_buttons),
                    _ => clicked_buttons.buttons.push(button_label.0.clone()),
                }

                for mut text in text_query.iter_mut() {
                    text.sections[0].value = clicked_buttons.to_number_string();
                }

                println!(
                    "Button {} clicked! Current sequence: {}",
                    button_label.0, clicked_buttons.to_number_string()
                );
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into(); // Lighter color on hover
                border_color.0 = Color::WHITE; // Change border color on hover
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into(); // Default button color
                border_color.0 = BORDER_COLOR; // Default border color
            }
        }
    }
}
