use bevy::prelude::*;
use crate::component::{ButtonLabel, SequenceDisplay};
use crate::constants::{NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON, PURPLE};
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
