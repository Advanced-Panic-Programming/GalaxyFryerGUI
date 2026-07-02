mod app_states;
mod setup_simulation;
mod galaxy_view;
mod legend;
mod planet_view;
mod input_handler;
mod setup_orchestrator;
mod app_state_manager;
mod cutscene;
mod pause_menu;
mod communication;
mod simulation_end;
mod log;
mod manual_mode;

use bevy::prelude::*;
use bevy::window::{WindowMode};
use crate::galaxy_view::plugin::GalaxyViewPlugin;
use crate::input_handler::plugin::InputHandlerPlugin;
use crate::setup_simulation::plugin::SetupSimulationPlugin;
use crate::planet_view::plugin::PlanetViewPlugin;
use crate::setup_orchestrator::plugin::SetupOrchestratorPlugin;
use crate::pause_menu::plugin::PauseMenuPlugin;
use crate::app_state_manager::plugin::AppStateManagerPlugin;
use crate::app_states::AppState;
use crate::communication::plugin::CommunicationPlugin;
use crate::cutscene::plugin::CutscenePlugin;
use crate::legend::plugin::LegendPlugin;
use crate::log::plugin::LogPlugin;
use crate::manual_mode::plugin::ManualModePlugin;
use crate::simulation_end::plugin::SimulationEndPlugin;

fn main() {

    App::new()
        // ===== Default Plugins ===== (MUST be before everything else)
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    title: "GalaxyFryer".to_string(),
                    ..default()
                }),
                ..default()
            })
        )
        // ===== Plugins =====
        .add_plugins(CommunicationPlugin)
        .add_plugins(AppStateManagerPlugin)
        .add_plugins(SetupSimulationPlugin)
        .add_plugins(SetupOrchestratorPlugin)
        .add_plugins(PauseMenuPlugin)
        .add_plugins(LegendPlugin)
        .add_plugins(LogPlugin)
        .add_plugins(ManualModePlugin)
        .add_plugins(InputHandlerPlugin)
        .add_plugins(GalaxyViewPlugin)
        .add_plugins(PlanetViewPlugin)
        .add_plugins(CutscenePlugin)
        .add_plugins(SimulationEndPlugin)
        // AppStates init
        .init_state::<AppState>()
        // PostStartup -> Sets fullscreen after startup bc bevy is bugged
        .add_systems(PostStartup, set_fullscreen)
        .run();
}

fn set_fullscreen(mut windows: Query<&mut Window>) {
    for mut win in windows.iter_mut() {
        win.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
    }
}