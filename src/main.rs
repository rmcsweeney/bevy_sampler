use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, pbr::NotShadowCaster, prelude::*, render::{camera::RenderTarget, view::RenderLayers, Render},};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(init_world, spawn_view_model, spawn_crosshair))
        .add_systems(Update, (keyboard_events, player_move, player_look))
        .run();
    println!("Hello, world!");
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002),)
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

const DEFAULT_RENDER_LAYER: usize = 0;
const VIEW_MODEL_RENDER_LAYER: usize=1;


fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let arm = meshes.add(Cuboid::new(0.1,0.1,0.5));
    let arm_material = materials.add(Color::srgb(0.9, 0.1, 0.4));

    commands.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(0., 1., 0.),
        Visibility::default(),
    )).with_children(|parent| {
        parent.spawn((
            WorldModelCamera,
            Camera3d::default(),
            Projection::from(PerspectiveProjection {
                fov: 90.0_f32.to_radians(),
                ..default()
            }),
            RenderLayers::layer(DEFAULT_RENDER_LAYER),
        ));

        parent.spawn((
            Camera3d::default(),
            Camera {
                order: 1,
                ..default()
            },
            Projection::from(PerspectiveProjection {
                fov: 70.0_f32.to_radians(),
                ..default()
            }),
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        ));

        parent.spawn((
            Mesh3d(arm),
            MeshMaterial3d(arm_material),
            Transform::from_xyz(0.2, -0.1, -0.25),
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            NotShadowCaster,
        ));
        
        

    });
}


fn player_move(time: Res<Time>, s: Single<&mut Transform, With<Player>>, buttons: ResMut<ButtonInput<KeyCode>>) {
    //World space, x z is the ground, y is up. Default facing angle is that Z is forward. 
    // Y  Z
    // | / 
    // |/
    // *------X

    let mut input_vec = Vec3::new(0.,0.,0.);
    let mut transform = s.into_inner();

    //Adding to input_vec as if in default facing. 
    if buttons.pressed(KeyCode::KeyW)        { input_vec.z += 1.*time.delta().as_secs_f32(); }
    if buttons.pressed(KeyCode::KeyS)        { input_vec.z -= 1.*time.delta().as_secs_f32(); }

    if buttons.pressed(KeyCode::KeyD)        { input_vec.x += 1.*time.delta().as_secs_f32(); }
    if buttons.pressed(KeyCode::KeyA)        { input_vec.x -= 1.*time.delta().as_secs_f32(); }

    if buttons.pressed(KeyCode::Space)       { input_vec.y += 1.*time.delta().as_secs_f32(); }
    if buttons.pressed(KeyCode::ControlLeft) { input_vec.y -= 1.*time.delta().as_secs_f32(); }
    
    let ground_facing_vector = Vec2::new(transform.forward().x, transform.forward().z).normalize(); //Defaults to 0, -1

    transform.translation.z += (ground_facing_vector.y * input_vec.z + input_vec.x * ground_facing_vector.x) * time.delta().as_secs_f32() * 1000.; // north south  
    transform.translation.x += (ground_facing_vector.x * input_vec.z - input_vec.x * ground_facing_vector.y) * time.delta().as_secs_f32() * 1000.; // east west 

    transform.translation.y += input_vec.y * time.delta().as_secs_f32() * 1000.; // Up Down, indpendent of camera
}

fn player_look(accumulated_mouse_motion: Res<AccumulatedMouseMotion>, mut player: Query<(&mut Transform, &mut CameraSensitivity), With<Player>>) {
    let Ok((mut transform, camera_sensitivity)) = player.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn keyboard_events(keys: Res<ButtonInput<KeyCode>>) {
    for key in keys.get_just_pressed() {
        println!("{:?} was just pressed", key);
    }
}

fn spawn_crosshair(mut commands: Commands, assets: Res<AssetServer<>>) {
    let crosshair = assets.load("crosshair007.png");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            RenderLayers::layer(2),
        )).with_children(|parent| {
            parent.spawn((
                ImageNode{
                    image: crosshair,
                    ..default()
                },
            ));
        });

}

fn init_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.,1.,1.))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(0., 0.5, -2.),
    ));


    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10., 10.).subdivisions(0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0., 0., 0.)
    ));

    commands.spawn((
        PointLight{
            shadows_enabled: true,
            intensity: 100000.,
            ..default()
        },
        Transform::from_xyz(5., 8., 5.)
    ));

    

}

