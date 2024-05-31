use std::default;

use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
struct CameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraMarker,
    ));
}

fn setup_plain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(10.0, 1.0, 10.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn update_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &CameraMarker)>,
    mut motion_evr: EventReader<MouseMotion>,
    window: Query<&Window>,
) {
    if window.single().cursor_position().is_some() {
        let (mut camera, _) = camera.single_mut();
        for ev in motion_evr.read() {
            camera.rotate_y(ev.delta.x / 100.0);
            camera.rotate_x(ev.delta.y / 100.0);
        }
        let cloned_camera = camera.clone();
        if keys.pressed(KeyCode::KeyW) {
            camera.translation += Vec3::from_array(cloned_camera.forward().to_array()) / 50.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            camera.translation += Vec3::from_array(cloned_camera.left().to_array()) / 50.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            camera.translation += Vec3::from_array(cloned_camera.back().to_array()) / 50.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            camera.translation += Vec3::from_array(cloned_camera.right().to_array()) / 50.0;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_plain))
        .add_systems(Update, update_camera)
        .run();
}
