use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;

use crate::assets::{FontAssets, MaterialAssets};

use super::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
  fn build(&self, app: &mut AppBuilder) {
    AssetLoader::new(GameState::Loading, GameState::Running)
      .with_collection::<FontAssets>()
      .init_resource::<MaterialAssets>()
      .build(app);
  }
}
