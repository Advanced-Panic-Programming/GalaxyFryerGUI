use bevy::prelude::*;
use common_game::utils::ID;
use galaxy_fryer::orchestrator::{OrchestratorToGUI::*};
use crate::setup_orchestrator::resources::{FromOrchestrator, ToOrchestrator};
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
    mut simulation_end_writer: MessageWriter<ReceivedSimulationEnd>
){
    while let Ok(msg) = receiver.0.try_recv() {
        match msg {
            DefaultMessage => {}
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
            }
            SendAsteroidDestroyed{p_id} => {
                // Generate in-game event
                asteroid_destroyed_writer.write(ReceivedAsteroidDestroyed{planet_id: p_id});
                // Activate cutscene
                asteroid_destroyed_cutscene_writer.write(AsteroidDestroyedCutscene{planet_id: p_id});
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
            }
            SendExplorerBag{explorer_id, bag} => {
                explorer_bag_writer.write(ReceivedExplorerBag{ 
                    explorer_id,
                    explorer_bag: bag,
                });
            }
            SendKilledExplorer {explorer_id} => {
                explorer_kill_writer.write(ReceivedKilledExplorer{explorer_id});
            }
            SendSimulationEnd => {
                simulation_end_writer.write(ReceivedSimulationEnd);
            }
        }
    }
}