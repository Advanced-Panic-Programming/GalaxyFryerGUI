use bevy::prelude::*;
use galaxy_view::ui::plugin::GalaxyViewUiPlugin;
use galaxy_view::systems::*;
use crate::app_states::AppState::GalaxyView;
use crate::galaxy_view;

pub struct GalaxyViewPlugin;

impl Plugin for GalaxyViewPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(GalaxyViewUiPlugin)
            // OnEnter Systems
            .add_systems(OnEnter(GalaxyView), (
                spawn_planets,
            ))
            // Update Systems
            .add_systems(Update, (
                update_planets,
                execute_animations,
                bound_explorer_arrows,
                // update_explorer_arrows_planet_binding,
                // animate_explorer_arrows,
            ).run_if(in_state(GalaxyView)))
            // OnExit Systems
            .add_systems(OnExit(GalaxyView), cleanup)
        ;
    }
}