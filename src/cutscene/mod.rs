use common_game::utils::ID;

pub mod plugin;
mod systems;
mod resources;
pub mod messages;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CutSceneType {
    PlanetDestroyed(ID),
    AsteroidDestroyed,
}
