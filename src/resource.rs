use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use bevy::prelude::*;

use crate::component::Position;

pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;

#[derive(Debug, Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Debug)]
pub struct FoodSpawnTimer(Timer);

impl Default for FoodSpawnTimer {
  fn default() -> Self {
    Self(Timer::new(Duration::from_secs(5), true))
  }
}

impl Deref for FoodSpawnTimer {
  type Target = Timer;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for FoodSpawnTimer {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[derive(Debug)]
pub struct SnakeMoveTimer(Timer);

impl Deref for SnakeMoveTimer {
  type Target = Timer;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for SnakeMoveTimer {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Default for SnakeMoveTimer {
  fn default() -> Self {
    Self(Timer::new(Duration::from_millis(500), true))
  }
}

#[derive(Debug, Default)]
pub struct LastTailPosition(pub Option<Position>);
