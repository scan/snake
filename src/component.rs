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
