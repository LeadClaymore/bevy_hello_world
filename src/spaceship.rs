use bevy::{math::VectorSpace, prelude::*};
use crate::movement::{MovingObjectBundle, Velocity, Acceleration};
use crate::asset_loader::SceneAsset;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<SceneAsset>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        acceleration: Acceleration::new(Vec3::ZERO),
        model: SceneBundle {
            scene: asset_server.spaceship.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        }
    });

    // commands.spawn(MovingObjectBundle {
    //     velocity: Velocity {
    //         value: Vec3 {x: 0.0, y: 0.0, z: 0.0},
    //     },
    //     acceleration: Acceleration::new(Vec3::ZERO),
    //     model: SceneBundle {
    //         scene: asset_server.load("Planet.glb#Scene0"),
    //         transform: Transform::from_translation(Vec3 {x: 0.0, y: 0.0, z: 0.0}),
    //         ..default()
    //     }
    // });
}