use bevy::prelude::Component;
use common_game::utils::ID;

#[derive(Component)]
pub struct SpawnedByPlanetView;

#[derive(Component)]
pub struct CornerPlanet {
    pub planet_id: ID,
}

#[derive(Component)]
pub struct Rocket {
    pub planet_id: ID,
}

#[derive(Component)]
pub struct Explorer {
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct ExplorerBag {
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct EnergyCells {
    pub planet_id: ID,
}