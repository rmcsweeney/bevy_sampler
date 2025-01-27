use bevy::prelude::*;

pub mod gameUi;
pub mod crosshair;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, crosshair::spawn_crosshair);
    }
}