use bevy::prelude::*;
use crate::log::resources::LogMessage;
use galaxy_fryer::app::gui_protocol::OrchestratorToGUI::*;
use crate::setup_orchestrator::resources::FromOrchestrator;
use crate::galaxy_view::messages::*;

/// This function manages the orchestrator messages and generates the corresponding events 
/// that will be handled by the relative modules 
pub fn receive_from_orchestrator(
    receiver: Res<FromOrchestrator>,
    // Planet
    mut planet_state_writer: MessageWriter<ReceivedPlanetState>,
    mut planet_destroyed_writer: MessageWriter<ReceivedPlanetDestroyed>,
    mut planet_destroyed_cutscene_writer: MessageWriter<PlanetDestroyedCutscene>,
    mut asteroid_destroyed_writer: MessageWriter<ReceivedAsteroidDestroyed>,
    mut asteroid_destroyed_cutscene_writer: MessageWriter<AsteroidDestroyedCutscene>,
    mut planet_generate_writer: MessageWriter<ReceivedPlanetGenerate>,
    mut planet_combine_writer: MessageWriter<ReceivedPlanetCombine>,
    // Explorer
    mut explorer_position_writer: MessageWriter<ReceivedExplorerPosition>,
    mut explorer_moved_writer: MessageWriter<ReceivedExplorerMove>,
    mut explorer_bag_writer: MessageWriter<ReceivedExplorerBag>,
    mut explorer_kill_writer: MessageWriter<ReceivedKilledExplorer>,
    // Simulation
    mut manual_mode_writer: MessageWriter<ReceivedManualModeAck>,
    mut automatic_mode_writer: MessageWriter<ReceivedAutomaticModeAck>,
    mut simulation_end_writer: MessageWriter<ReceivedSimulationEnd>,
    // Log
    mut log: MessageWriter<LogMessage>,
){
    while let Ok(msg) = receiver.0.try_recv() {
        match msg {
            DefaultMessage => {}
            ManualModeAck => {
                // Generates the corresponding in game event
                manual_mode_writer.write(ReceivedManualModeAck);
                // Generates a 'LogMessage' event handled by the LogPlugin
                log.write(LogMessage::manual_mode_ack());
            }
            AutomaticModeAck => {
                automatic_mode_writer.write(ReceivedAutomaticModeAck);
                log.write(LogMessage::automatic_mode_ack());
            }
            SendSunrayAck { p_id } => {
                log.write(LogMessage::send_sunray_ack(p_id));
            }
            SendAsteroidAck { p_id } => {
                log.write(LogMessage::send_asteroid_ack(p_id));
            }
            SendPlanetState{p_id, planet_state} => {
                planet_state_writer.write(ReceivedPlanetState{
                    planet_id: p_id,
                    dummy_planet_state: planet_state
                });
            }
            SendPlanetDestroyed{p_id} => {
                // Generate in-game event
                planet_destroyed_writer.write(ReceivedPlanetDestroyed{planet_id: p_id});
                // Activate cutscene
                planet_destroyed_cutscene_writer.write(PlanetDestroyedCutscene{planet_id: p_id});
                //Log
                log.write(LogMessage::planet_destroyed(p_id));
            }
            SendAsteroidDestroyed{p_id} => {
                // Generate in-game event
                asteroid_destroyed_writer.write(ReceivedAsteroidDestroyed);
                // Activate cutscene
                asteroid_destroyed_cutscene_writer.write(AsteroidDestroyedCutscene);
                // Log
                log.write(LogMessage::asteroid_destroyed(p_id));
            }
            SendPlanetGenerate{planet_id, generate} => {
                planet_generate_writer.write(ReceivedPlanetGenerate{
                    planet_id,
                    generate,
                });
            }
            SendPlanetCombine{planet_id, combine} => {
                planet_combine_writer.write(ReceivedPlanetCombine{
                    planet_id,
                    combine,
                });
            }
            SendExplorerPosition{explorer_id, planet_id} => {
                explorer_position_writer.write(ReceivedExplorerPosition{
                    explorer_id,
                    planet_id,
                });
            }
            SendExplorerMoved {explorer_id, planet_id} => {
                explorer_moved_writer.write(ReceivedExplorerMove{ 
                    explorer_id,
                    planet_id,
                });
                log.write(LogMessage::explorer_moved(explorer_id, planet_id));
            }
            SendExplorerBag{explorer_id, bag} => {
                explorer_bag_writer.write(ReceivedExplorerBag{ 
                    explorer_id,
                    explorer_bag: bag.clone(),
                });
                // Prints in the log the explorer state (only explorer 2 has a state)
                // The State is sent only the first time the explorer enters the state or when it changes state so that it doesn't print the same state multiple times
                if let Some(state) = bag.get_state() {
                    log.write(LogMessage::explorer_state(explorer_id, state));
                }
            }
            SendKilledExplorer {explorer_id} => {
                explorer_kill_writer.write(ReceivedKilledExplorer{explorer_id});
                log.write(LogMessage::explorer_killed(explorer_id));
            }
            SendSimulationEnd => {
                simulation_end_writer.write(ReceivedSimulationEnd);
                log.write(LogMessage::simulation_end());
            }
        }
    }
}