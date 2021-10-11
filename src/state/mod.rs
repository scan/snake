mod loading;
mod score;

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

pub use loading::LoadingPlugin;
pub use score::{Score, ScorePlugin};
