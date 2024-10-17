use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAsset {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missile: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAsset>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAsset>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAsset {
        asteroid: asset_server.load("Asteroid.glb#Scene0"),
        spaceship: asset_server.load("Spaceship.glb#Scene0"),
        missile: asset_server.load("Missile.glb#Scene0"),
    }
}