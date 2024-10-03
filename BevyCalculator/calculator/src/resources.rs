// src/resources.rs
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct ClickedButtons {
    pub buttons: Vec<String>,
}

impl ClickedButtons {
    pub fn to_number_string(&self) -> String {
        self.buttons.join("") // Convert to a single string
    }
}
