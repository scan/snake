use bevy::prelude::*;

use crate::assets::FontAssets;

use super::GameState;

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
struct ScoreBoard;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .insert_resource(Score::default())
      .add_system_set(SystemSet::on_enter(GameState::Starting).with_system(setup_score.system()))
      .add_system(update_score.system());
  }
}

fn setup_score(mut commands: Commands, fonts: Res<FontAssets>, score: Res<Score>) {
  commands
    .spawn_bundle(TextBundle {
      style: Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: Rect {
          bottom: Val::Px(5.0),
          right: Val::Px(15.0),
          ..Default::default()
        },
        ..Default::default()
      },
      text: Text::with_section(
        *score,
        TextStyle {
          font: fonts.font.clone(),
          font_size: 50.0,
          color: Color::WHITE,
        },
        TextAlignment {
          horizontal: HorizontalAlign::Center,
          ..Default::default()
        },
      ),
      ..Default::default()
    })
    .insert(ScoreBoard);
}

fn update_score(mut query: Query<&mut Text, With<ScoreBoard>>, score: Res<Score>) {
  for mut text in query.iter_mut() {
    text.sections[0].value = (*score).into();
  }
}
