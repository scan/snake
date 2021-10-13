use bevy::prelude::*;

use crate::assets::FontAssets;

use super::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(setup_text.system()))
      .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(wait_for_space.system()))
      .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(remove_text.system()));
  }
}

struct GameOverText;

fn setup_text(
  mut commands: Commands,
  fonts: Res<FontAssets>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands
    .spawn_bundle(NodeBundle {
      style: Style {
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexEnd,
        ..Default::default()
      },
      material: materials.add(Color::NONE.into()),
      ..Default::default()
    })
    .with_children(|parent| {
      parent.spawn_bundle(TextBundle {
        style: Style {
          align_self: AlignSelf::Center,
          ..Default::default()
        },
        text: Text::with_section(
          "Game Over",
          TextStyle {
            font: fonts.font.clone(),
            font_size: 75.0,
            color: Color::WHITE,
          },
          TextAlignment {
            horizontal: HorizontalAlign::Center,
            vertical: VerticalAlign::Center,
            ..Default::default()
          },
        ),
        ..Default::default()
      });
    })
    .insert(GameOverText);
}

fn wait_for_space(mut app_state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    app_state.pop().expect("could not pop state");
  }
}

fn remove_text(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
  for entity in query.iter() {
    commands.entity(entity).despawn_recursive();
  }
}
