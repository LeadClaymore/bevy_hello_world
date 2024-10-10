// Zymartu hello world ep2
// https://youtu.be/R-u1EY9fOJQ
use bevy::prelude::*;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use camera::CameraPlugin;

mod debug;
mod movement;
mod spaceship;
mod camera;

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
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        //user plugins
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}

