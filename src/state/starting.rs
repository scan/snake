use bevy::prelude::*;

use super::GameState;

pub struct StartUpPlugin;

impl Plugin for StartUpPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_system_set(SystemSet::on_enter(GameState::Starting).with_system(switch_state.system()));
  }
}

fn switch_state(mut app_state: ResMut<State<GameState>>) {
  app_state
    .set(GameState::Running)
    .expect("How did this happen?");
}
