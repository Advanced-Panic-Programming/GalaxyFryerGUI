use bevy::prelude::*;
use crate::app_states::AppState::*;
use crate::log::ui::resources::{DisplayedLogEntries, LogAutoScroll, PreviousPosition};
use crate::log::ui::systems::*;
use crate::log::ui::components::LogUI;

pub struct LogUIPlugin;

impl Plugin for LogUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LogAutoScroll>()
            .init_resource::<DisplayedLogEntries>()
            .insert_resource(PreviousPosition { previous: 0.0, })
            // .init_resource::<PreviousPosition>()
            // OnEnter
            // The app flow is PauseMenu -> GalaxyView <-> PlanetView <-> PauseMenu
            .add_systems(OnEnter(GalaxyView), spawn_log_ui.run_if(not(any_with_component::<LogUI>))) // fist state after 'play' is pressed
            // the run_if ensures that the log will be spawned only the first time we enter GalaxyView
            // or if it has been deleted (i.e. GalaxyView -> PauseMenu -> GalaxyView)
            .add_systems(OnEnter(PauseMenu), cleanup_log_ui)
            // .add_systems(OnEnter(PlanetView), spawn_log_ui)
            // Update
            .add_systems(Update, 
                (
                    update_log_ui,
                    scroll_log_ui,
                ).run_if(in_state(GalaxyView).or(in_state(PlanetView))),
            )
            // OnExit
            // .add_systems(OnExit(GalaxyView), cleanup_log_ui)//.run_if(resource_removed::<crate::log::resources::LogStore>))
            // .add_systems(OnExit(PlanetView), cleanup_log_ui)//.run_if(resource_removed::<crate::log::resources::LogStore>))
        ;
    }
}