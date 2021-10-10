use bevy::prelude::*;
use direction::CardinalDirection;
use rand::prelude::random;
use std::time::Duration;

fn main() {
  App::build()
    .insert_resource(WindowDescriptor {
      title: "Snake!".to_string(),
      width: 500.0,
      height: 500.0,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(SnakeMoveTimer::default())
    .insert_resource(SnakeSegments::default())
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
    .add_system(snake_movement.system())
    .add_system(size_scaling.system())
    .add_system(position_translation.system())
    .add_system(food_spawner.system())
    .add_system(snake_timer.system())
    .run();
}

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Debug)]
struct SnakeHead {
  direction: CardinalDirection,
}

impl Default for SnakeHead {
  fn default() -> Self {
    SnakeHead {
      direction: CardinalDirection::North,
    }
  }
}

#[derive(Debug)]
struct SnakeSegment;

#[derive(Debug, Default)]
struct SnakeSegments(Vec<Entity>);

#[derive(Debug)]
struct Materials {
  head_material: Handle<ColorMaterial>,
  segment_material: Handle<ColorMaterial>,
  food_material: Handle<ColorMaterial>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
  x: i32,
  y: i32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Size {
  width: f32,
  height: f32,
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
struct Food;

#[derive(Debug)]
struct FoodSpawnTimer(Timer);

impl Default for FoodSpawnTimer {
  fn default() -> Self {
    Self(Timer::new(Duration::from_secs(1), true))
  }
}

struct SnakeMoveTimer(Timer);

impl Default for SnakeMoveTimer {
  fn default() -> Self {
    Self(Timer::new(Duration::from_millis(500), true))
  }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
  commands.insert_resource(Materials {
    head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
    food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
  });
}

fn spawn_snake(
  mut commands: Commands,
  materials: Res<Materials>,
  mut segments: ResMut<SnakeSegments>,
) {
  segments.0 = vec![
    commands
      .spawn_bundle(SpriteBundle {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        ..Default::default()
      })
      .insert(SnakeHead::default())
      .insert(Position { x: 3, y: 3 })
      .insert(Size::square(0.8))
      .id(),
    spawn_segment(
      commands,
      &materials.segment_material,
      Position { x: 3, y: 2 },
    ),
  ]
}

fn snake_movement(
  keyboard_input: Res<Input<KeyCode>>,
  snake_timer: ResMut<SnakeMoveTimer>,
  mut heads: Query<(Entity, &mut SnakeHead)>,
  mut positions: Query<&mut Position>,
) {
  if let Some((head_entity, mut head)) = heads.iter_mut().next() {
    if let Ok(mut head_pos) = positions.get_mut(head_entity) {
      let direction = if keyboard_input.pressed(KeyCode::Up) {
        CardinalDirection::North
      } else if keyboard_input.pressed(KeyCode::Down) {
        CardinalDirection::South
      } else if keyboard_input.pressed(KeyCode::Left) {
        CardinalDirection::West
      } else if keyboard_input.pressed(KeyCode::Right) {
        CardinalDirection::East
      } else {
        head.direction
      };

      if direction != head.direction.opposite() {
        head.direction = direction;
      }

      if !snake_timer.0.finished() {
        return;
      }

      match &head.direction {
        CardinalDirection::North => head_pos.y += 1,
        CardinalDirection::South => head_pos.y -= 1,
        CardinalDirection::West => head_pos.x -= 1,
        CardinalDirection::East => head_pos.x += 1,
      };
    }
  }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
  if let Some(window) = windows.get_primary() {
    for (sprite_size, mut sprite) in q.iter_mut() {
      sprite.size = Vec2::new(
        sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
        sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
      )
    }
  }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
  fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
  }

  if let Some(window) = windows.get_primary() {
    for (pos, mut transform) in q.iter_mut() {
      transform.translation = Vec3::new(
        convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
        convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
        0.0,
      )
    }
  }
}

fn food_spawner(
  mut commands: Commands,
  materials: Res<Materials>,
  time: Res<Time>,
  mut timer: Local<FoodSpawnTimer>,
) {
  if timer.0.tick(time.delta()).finished() {
    commands
      .spawn_bundle(SpriteBundle {
        material: materials.food_material.clone(),
        ..Default::default()
      })
      .insert(Food)
      .insert(Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
      })
      .insert(Size::square(0.8));
  }
}

fn snake_timer(time: Res<Time>, mut snake_timer: ResMut<SnakeMoveTimer>) {
  snake_timer.0.tick(time.delta());
}

fn spawn_segment(
  mut commands: Commands,
  material: &Handle<ColorMaterial>,
  position: Position,
) -> Entity {
  commands
    .spawn_bundle(SpriteBundle {
      material: material.clone(),
      ..Default::default()
    })
    .insert(SnakeSegment)
    .insert(position)
    .insert(Size::square(0.65))
    .id()
}
