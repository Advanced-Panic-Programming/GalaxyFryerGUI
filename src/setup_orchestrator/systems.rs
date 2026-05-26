use std::collections::HashMap;
use std::thread;
use bevy::prelude::*;
use crate::setup_orchestrator::resources::*;
use galaxy_fryer::app::orchestrator::Orchestrator;
use galaxy_fryer::app::gui_protocol::GUIToOrchestrator;
use galaxy_fryer::app::planet_info::PlanetInfo;
use galaxy_fryer::app::explorer_info::ExplorerInfo;
use common_game::utils::ID;
use crate::app_state_manager::messages::SetupOrchestratorCompleted;

pub fn setup_orchestrator(mut commands: Commands) {

    // Initialize empty hashmaps
    let galaxy:HashMap<ID, PlanetInfo> = HashMap::new();
    let explorers: HashMap<ID, ExplorerInfo> = HashMap::new();

    // Communication with/from Orchestrator channels
    let (gui_to_orch_s, gui_to_orch_r) = crossbeam_channel::unbounded();
    let (orch_to_gui_s, orch_to_gui_r) = crossbeam_channel::unbounded();

    // Spawn Orchestrator thread
    thread::spawn(move || {
        let mut orchestrator = Orchestrator::new(
            galaxy,
            explorers,
            orch_to_gui_s,
            gui_to_orch_r,
        );
        orchestrator.initialize_galaxy();
    });

    let _ = gui_to_orch_s.send(GUIToOrchestrator::DefaultMessage); //TODO REMOVE

    // Adding channels as Bevy resources in order to use them in the GUI
    commands.insert_resource(ToOrchestrator(gui_to_orch_s));
    commands.insert_resource(FromOrchestrator(orch_to_gui_r));
}

pub fn setup_orchestrator_completed(mut writer: MessageWriter<SetupOrchestratorCompleted>) {
    writer.write(SetupOrchestratorCompleted);
}