use bevy::prelude::*;

pub fn spawn_crosshair(mut commands: Commands, assets: Res<AssetServer>){

    commands.spawn((
        RenderLayers::VIEW_MODEL_RENDER_LAYER,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
        }
    )).with_children(|parent|{
        parent.spawn((
            ImageNode{
                AssetServer.load("assets/crosshair007.png"),
                ..default()
            }
        ));
    })

}