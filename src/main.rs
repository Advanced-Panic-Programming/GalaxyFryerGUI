mod app_states;
mod setup_simulation;
mod galaxy_view;
mod resources_manager;
mod legend;
mod planet_view;
mod common_systems;
mod input_handler;
mod setup_orchestrator;
mod app_state_manager;
mod cutscene;
mod pause_menu;
mod communication;
mod simulation_end;

use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use crate::galaxy_view::plugin::GalaxyViewPlugin;
use crate::input_handler::plugin::InputHandlerPlugin;
use crate::setup_simulation::plugin::SetupSimulationPlugin;
use crate::resources_manager::plugin::ResourceManagerPlugin;
use crate::planet_view::plugin::PlanetViewPlugin;
use crate::setup_orchestrator::plugin::SetupOrchestratorPlugin;
use crate::pause_menu::plugin::PauseMenuPlugin;
use crate::app_state_manager::plugin::AppStateManagerPlugin;
use crate::app_states::AppState;
use crate::communication::plugin::CommunicationPlugin;
use crate::cutscene::plugin::CutscenePlugin;
use crate::legend::plugin::LegendPlugin;
use crate::simulation_end::plugin::SimulationEndPlugin;

fn main() {

    App::new()
        // ===== Plugins ===== (MUST be before everything else)
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    resolution: WindowResolution::new(1920, 1080),
                    title: "AirFryer".to_string(),
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
        .add_plugins(ResourceManagerPlugin)
        .add_plugins(PauseMenuPlugin)
        .add_plugins(LegendPlugin)
        .add_plugins(InputHandlerPlugin)
        .add_plugins(GalaxyViewPlugin)
        .add_plugins(PlanetViewPlugin)
        .add_plugins(CutscenePlugin)
        .add_plugins(SimulationEndPlugin)
        // States init
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