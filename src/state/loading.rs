use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;

use crate::assets::{FontAssets, MaterialAssets};

use super::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
  fn build(&self, app: &mut AppBuilder) {
    AssetLoader::new(GameState::Loading, GameState::Starting)
      .with_collection::<FontAssets>()
      .init_resource::<MaterialAssets>()
      .build(app);

    app.add_startup_system(setup.system());
  }
}

fn setup(mut commands: Commands) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.spawn_bundle(UiCameraBundle::default());
}
