use bevy::prelude::*;

pub fn update_keypresses(keys: Res<ButtonInput<KeyCode>>) {
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::KeyW =>{
                println!("W pressed");
            }
        }
    }
}