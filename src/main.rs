mod assets;
mod component;
mod state;

use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use direction::CardinalDirection;
use rand::prelude::random;

use crate::{assets::*, component::*, state::*};

fn main() {
  let mut app = App::build();

  AssetLoader::new(GameState::Loading, GameState::Loading)
    .with_collection::<FontAssets>()
    .init_resource::<MaterialAssets>()
    .build(&mut app);

  app
    .insert_resource(WindowDescriptor {
      title: "Snake!".to_string(),
      width: 500.0,
      height: 500.0,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(SnakeMoveTimer::default())
    .insert_resource(SnakeSegments::default())
    .insert_resource(LastTailPosition::default())
    .add_event::<GrowthEvent>()
    .add_event::<GameOverEvent>()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
    .add_system(snake_movement.system())
    .add_system(size_scaling.system())
    .add_system(position_translation.system())
    .add_system(food_spawner.system())
    .add_system(snake_timer.system())
    .add_system(snake_eating.system())
    .add_system(snake_growth.system())
    .add_system(game_over.system())
    .run();
}

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(
  mut commands: Commands,
  materials: Res<MaterialAssets>,
  mut segments: ResMut<SnakeSegments>,
) {
  segments.0 = vec![
    commands
      .spawn_bundle(SpriteBundle {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        ..Default::default()
      })
      .insert(SnakeHead::default())
      .insert(Position { x: 3, y: 3 })
      .insert(component::Size::square(0.8))
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
  segments: ResMut<SnakeSegments>,
  mut last_tail_position: ResMut<LastTailPosition>,
  mut heads: Query<(Entity, &mut SnakeHead)>,
  mut positions: Query<&mut Position>,
  mut game_over_events: EventWriter<GameOverEvent>,
) {
  if let Some((head_entity, mut head)) = heads.iter_mut().next() {
    let segment_positions = segments
      .0
      .iter()
      .filter_map(|e| positions.get_mut(*e).ok().map(|p| p.to_owned()))
      .collect::<Vec<Position>>();

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

      if !snake_timer.finished() {
        return;
      }

      match &head.direction {
        CardinalDirection::North => head_pos.y += 1,
        CardinalDirection::South => head_pos.y -= 1,
        CardinalDirection::West => head_pos.x -= 1,
        CardinalDirection::East => head_pos.x += 1,
      };

      if head_pos.x < 0
        || head_pos.x as u32 >= ARENA_WIDTH
        || head_pos.y < 0
        || head_pos.y as u32 >= ARENA_HEIGHT
      {
        game_over_events.send(GameOverEvent);
      }

      if segment_positions.contains(&head_pos) {
        game_over_events.send(GameOverEvent);
      }

      segment_positions
        .iter()
        .zip(segments.0.iter().skip(1))
        .for_each(|(pos, segment)| {
          if let Ok(mut segment_position) = positions.get_mut(*segment) {
            *segment_position = *pos;
          }
        });

      last_tail_position.0 = segment_positions.last().copied();
    }
  }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&component::Size, &mut Sprite)>) {
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
        transform.translation.z,
      )
    }
  }
}

fn food_spawner(
  mut commands: Commands,
  materials: Res<MaterialAssets>,
  time: Res<Time>,
  mut timer: Local<FoodSpawnTimer>,
) {
  if timer.tick(time.delta()).finished() {
    commands
      .spawn_bundle(SpriteBundle {
        material: materials.food_material.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
      })
      .insert(Food)
      .insert(Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
      })
      .insert(component::Size::square(0.8));
  }
}

fn snake_timer(time: Res<Time>, mut snake_timer: ResMut<SnakeMoveTimer>) {
  snake_timer.tick(time.delta());
}

fn spawn_segment(
  mut commands: Commands,
  material: &Handle<ColorMaterial>,
  position: Position,
) -> Entity {
  commands
    .spawn_bundle(SpriteBundle {
      material: material.clone(),
      transform: Transform::from_xyz(0.0, 0.0, 2.0),
      ..Default::default()
    })
    .insert(SnakeSegment)
    .insert(position)
    .insert(component::Size::square(0.65))
    .id()
}

fn snake_eating(
  mut commands: Commands,
  snake_timer: ResMut<SnakeMoveTimer>,
  mut growth_events: EventWriter<GrowthEvent>,
  food_positions: Query<(Entity, &Position), With<Food>>,
  head_positions: Query<&Position, With<SnakeHead>>,
) {
  if !snake_timer.finished() {
    return;
  }

  for head_position in head_positions.iter() {
    for (entity, food_position) in food_positions.iter() {
      if food_position == head_position {
        commands.entity(entity).despawn();
        growth_events.send(GrowthEvent);
      }
    }
  }
}

fn snake_growth(
  commands: Commands,
  last_tail_position: Res<LastTailPosition>,
  mut segments: ResMut<SnakeSegments>,
  mut growth_reader: EventReader<GrowthEvent>,
  materials: Res<MaterialAssets>,
) {
  if growth_reader.iter().next().is_some() {
    if let Some(last_tail_position) = last_tail_position.0 {
      segments.0.push(spawn_segment(
        commands,
        &materials.segment_material,
        last_tail_position,
      ));
    }
  }
}

fn game_over(
  mut commands: Commands,
  mut reader: EventReader<GameOverEvent>,
  materials: Res<MaterialAssets>,
  snake_segments: ResMut<SnakeSegments>,
  food: Query<Entity, With<Food>>,
  segments: Query<Entity, With<SnakeSegment>>,
) {
  if reader.iter().next().is_some() {
    for entity in food.iter().chain(segments.iter()) {
      commands.entity(entity).despawn();
    }
    spawn_snake(commands, materials, snake_segments);
  }
}
