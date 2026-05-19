use bevy::prelude::*;
use crate::pause_menu::systems::*;
use crate::app_states::AppState::PauseMenu;
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter systems
            .add_systems(OnEnter(PauseMenu), send_play_pressed)
            // Update systems
            // .add_systems(Update, send_play_pressed.run_if(in_state(PauseMenu)))
        ;
    }
}