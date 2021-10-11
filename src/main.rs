#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod component;
mod event;
mod resource;
mod state;

use bevy::{
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  prelude::*,
};
use direction::CardinalDirection;
use rand::prelude::random;
use state::{GamePlugins, GameState};

fn main() {
  App::build()
    .insert_resource(WindowDescriptor {
      title: "Snake!".to_string(),
      width: 500.0,
      height: 500.0,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_state(GameState::Loading)
    .add_plugins(DefaultPlugins)
    .add_plugins(GamePlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(LogDiagnosticsPlugin::default())
    .run();
}
