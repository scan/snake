mod game_over;
mod loading;
mod running;
mod score;
mod starting;

use bevy::prelude::PluginGroup;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GameState {
  Loading,
  Running,
  Starting,
  GameOver,
}

impl Default for GameState {
  fn default() -> Self {
    Self::Loading
  }
}

use game_over::GameOverPlugin;
use loading::LoadingPlugin;
use running::RunningPlugin;
use score::ScorePlugin;
use starting::StartUpPlugin;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
  fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
    group
      .add(LoadingPlugin)
      .add(RunningPlugin)
      .add(ScorePlugin)
      .add(StartUpPlugin)
      .add(GameOverPlugin);
  }
}
