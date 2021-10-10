use std::{
  ops::{Deref, DerefMut},
  time::Duration,
};

use bevy::prelude::*;
use direction::CardinalDirection;

#[derive(Debug)]
pub struct SnakeHead {
  pub direction: CardinalDirection,
}

impl Default for SnakeHead {
  fn default() -> Self {
    SnakeHead {
      direction: CardinalDirection::North,
    }
  }
}

#[derive(Debug)]
pub struct SnakeSegment;

#[derive(Debug, Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Size {
  pub width: f32,
  pub height: f32,
}

impl Size {
  pub fn square(x: f32) -> Self {
    Self {
      width: x,
      height: x,
    }
  }
}

#[derive(Debug)]
pub struct Food;

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
pub struct GrowthEvent;

#[derive(Debug)]
pub struct GameOverEvent;

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

#[derive(Debug, Default, Clone, Copy)]
pub struct Score(u64);

impl Score {
  pub fn increment(&mut self) {
    self.0 += 1;
  }

  pub fn reset(&mut self) {
    self.0 = 0;
  }
}

impl Into<String> for Score {
  fn into(self) -> String {
    self.0.to_string()
  }
}

#[derive(Debug)]
pub struct ScoreBoard;
