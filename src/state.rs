#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GameState {
  Loading,
  Paused,
  Running,
}

impl Default for GameState {
  fn default() -> Self {
    Self::Loading
  }
}
