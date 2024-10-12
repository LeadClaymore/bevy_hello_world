use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Startup, spawn_light);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

// this was for trying to fix an error where the spaceship was not collored in
// the fix was to change the intensity of ambient light from .12 to 300.0
fn spawn_light(mut commands: Commands) {
    commands.spawn( PointLightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        point_light: PointLight {
            intensity: 1500.0,
            range: 300.0,
            ..default()
        },
        ..default()
    });
}