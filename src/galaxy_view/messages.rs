use std::collections::HashSet;
use bevy::prelude::{Message, Resource};
use common_game::components::planet::DummyPlanetState;
use common_game::components::resource::{BasicResourceType, ComplexResourceType, ResourceType};
use common_game::utils::ID;
use galaxy_fryer::explorer::bag::BagView;

// Planet
#[derive(Message)]
pub struct ReceivedPlanetDestroyed {
    pub planet_id: ID,
}
#[derive(Message)]
pub struct PlanetDestroyedCutscene {
    pub planet_id: ID,
}
#[derive(Message)]
pub struct ReceivedAsteroidDestroyed{
    pub planet_id: ID,
}
#[derive(Message)]
pub struct AsteroidDestroyedCutscene{
    pub planet_id: ID,
}
#[derive(Message)]
pub struct ReceivedPlanetState{
    pub planet_id: ID,
    pub dummy_planet_state: DummyPlanetState, // rocket and energy cells
}
#[derive(Message)]
pub struct ReceivedPlanetGenerate {
    pub planet_id: ID,
    pub generate: HashSet<BasicResourceType>
}
#[derive(Message)]
pub struct ReceivedPlanetCombine {
    pub planet_id: ID,
    pub combine: HashSet<ComplexResourceType>
}
// Explorer
#[derive(Message)]
pub struct ReceivedExplorerPosition {
    pub explorer_id: ID,
    pub planet_id: ID,
}
#[derive(Message)]
pub struct ReceivedExplorerMove {
    pub explorer_id: ID,
    pub planet_id: ID,
}
#[derive(Message)]
pub struct ReceivedExplorerBag {
    pub explorer_id: ID,
    pub explorer_bag: BagView,
}
#[derive(Message)]
pub struct ReceivedKilledExplorer {
    pub explorer_id: ID,
}
// Simulation
#[derive(Message)]
pub struct ReceivedManualModeAck;
#[derive(Message)]
pub struct ReceivedAutomaticModeAck;
#[derive(Message)]
pub struct ReceivedSimulationEnd;