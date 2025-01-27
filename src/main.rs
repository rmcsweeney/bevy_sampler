use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, pbr::NotShadowCaster, prelude::*, render::{camera::RenderTarget, view::RenderLayers, Render}, utils::tracing::field::display};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (init_world, spawn_view_model,))
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
const CURSOR_RENDER_LAYER: usize = 2;

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
    let mut move_vec = Vec2::new(0.,0.);
    let mut move_vert = 0.0;//transform.translation.x -
    let mut transform = s.into_inner();

    if buttons.pressed(KeyCode::KeyW) { move_vec.x += 1.*time.delta().as_secs_f32(); }
    if buttons.pressed(KeyCode::KeyA) { move_vec.y -= 1.*time.delta().as_secs_f32(); }
    if buttons.pressed(KeyCode::KeyS) { move_vec.x -= 1.*time.delta().as_secs_f32(); }
    if buttons.pressed(KeyCode::KeyD) { move_vec.y += 1.*time.delta().as_secs_f32(); }
    if buttons.pressed(KeyCode::Space) { move_vert += 1.*time.delta().as_secs_f32(); }
    if buttons.pressed(KeyCode::ControlLeft) { move_vert -= 1.*time.delta().as_secs_f32(); }
    //transform.rotation.to_euler(XY)
    
    let f = Vec2::new(transform.forward().x, transform.forward().z).normalize();
    println!("Rotation: {:?}", f);
    //side mv z/x represent the horizontal component of movement in the z/x direction respectively--i.e. left input partial along the axis
    let side_mv_z = (1.0 - f.y.abs()) * move_vec.y;
    let side_mv_x = (1.0 - f.x.abs()) * move_vec.y;
    println!("Side z {:?} Side x {:?}", side_mv_z, side_mv_x);
    if f.y > 0.0 {
        transform.translation.z += (f.y * move_vec.x - side_mv_z) * time.delta().as_secs_f32() * 1000.;
    } else {
        transform.translation.z += (f.y * move_vec.x + side_mv_z) * time.delta().as_secs_f32() * 1000.;
    }
    if f.x > 0.0 {
        transform.translation.x += (f.x * move_vec.x + side_mv_x) * time.delta().as_secs_f32() * 1000.;
    } else {
        transform.translation.x += (f.x * move_vec.x - side_mv_x) * time.delta().as_secs_f32() * 1000.;
    }
    
    transform.translation.y += move_vert * time.delta().as_secs_f32() * 1000.;
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

fn init_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.,1.,1.))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(0., 0.5, 0.),
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

    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(2., 3., -3.).looking_at(Vec3::ZERO, Vec3::Y)
    // ));

}

