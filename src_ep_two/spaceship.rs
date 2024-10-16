use bevy::prelude::*;
use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        }
    });

    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: Vec3 {x: 0.0, y: 0.0, z: 0.0},
        },
        model: SceneBundle {
            scene: asset_server.load("Planet.glb#Scene0"),
            transform: Transform::from_translation(Vec3 {x: 0.0, y: 0.0, z: 0.0}),
            ..default()
        }
    });
}