use bevy::prelude::*;
use crate::app_states::AppState::GalaxyView;
use crate::galaxy_view::ui::systems::*;

pub struct GalaxyViewUiPlugin;

impl Plugin for GalaxyViewUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GalaxyView), spawn_galaxy_view_ui)
            .add_systems(OnExit(GalaxyView), despawn_galaxy_view_ui)
        ;
    }
}