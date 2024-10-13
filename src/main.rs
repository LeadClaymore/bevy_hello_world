// Zymartu hello world ep3
// https://youtu.be/R-u1EY9fOJQ

mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroids;
pub mod asset_loader;
//use crate::asset_loader::SceneAsset;

use bevy::prelude::*;

use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use camera::CameraPlugin;
use asteroids::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;


#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

fn main() {
    App::new()
        //built in stuff
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 300.0,
        })
        .add_plugins(DefaultPlugins)
        //user plugins
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(AssetLoaderPlugin)
        .run();
}

