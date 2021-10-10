use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(Debug, AssetCollection)]
pub struct FontAssets {
  #[asset(path = "BasicPixelFont2_cheezbitz.ttf")]
  pub font: Handle<Font>,
}

#[derive(Debug)]
pub struct MaterialAssets {
  pub head_material: Handle<ColorMaterial>,
  pub segment_material: Handle<ColorMaterial>,
  pub food_material: Handle<ColorMaterial>,
}

impl FromWorld for MaterialAssets {
  fn from_world(world: &mut World) -> Self {
    let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

    Self {
      head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
      segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
      food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    }
  }
}
