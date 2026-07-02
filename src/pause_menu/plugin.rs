use crate::app_states::AppState::PauseMenu;
use crate::pause_menu::systems::*;
use bevy::prelude::*;
use crate::setup_orchestrator::resources::ToOrchestrator;

/// This plugin handles the pause menu.
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter systems
            .add_systems(OnEnter(PauseMenu), (
                setup_pause_menu,
                init_planet_states.run_if(resource_exists::<ToOrchestrator>))
            )
            // Update systems
            .add_systems(Update, (animate_tittle, animate_logo, play_button_system, exit_button_system).run_if(in_state(PauseMenu)))
            // OnExit systems
            .add_systems(OnExit(PauseMenu), despawn_pause_menu);
    }
}
