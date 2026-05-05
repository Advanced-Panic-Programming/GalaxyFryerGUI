use bevy::prelude::*;
use std::result::Result;
use crate::app_states::AppState;
// All resources are defined here

#[derive(Resource, Default)]
pub struct LastState {
    pub state: AppState,
}

#[derive(Resource, Default)]
pub struct Galaxy {
    pub planets: Vec<Entity>,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct GalaxyOrbit {
    pub center: Vec3,
    pub a: f32,
    pub b: f32,
}

#[derive(Resource)]
pub struct PlanetsData {
    pub planets: Vec<PlanetInfo>,
}

pub struct PlanetInfo {
    pub name: String,
    pub index: usize,
    pub alive: bool,
    pub angle: f32,
    pub speed: f32,
    pub timer: Timer,
    pub sprite_path: String,
    pub destroyed_sprite_path: String,
}

#[derive(Resource, Default)]
pub struct SelectedPlanet {
    index: Option<usize>,
}

impl SelectedPlanet {
    pub fn set(&mut self, index: usize) -> Result<(), &str> {
        if index > 6 {
            Err("Out of bounds planet index")
        } else {
            self.index = Some(index);
            Ok(())
        }
    }

    pub fn get(&self) -> Option<usize> {
        self.index
    }

    pub fn clear(&mut self) { self.index = None; }
}

#[derive(Resource, Default)]
pub struct ExplorersData {
    pub explorer1: Explorer,
    pub explorer2: Explorer,
    last_cycle: bool,
}

impl ExplorersData {
    pub fn switch(&mut self) {
        self.last_cycle = !self.last_cycle;
    }
    
    pub fn get(&self) -> bool {
        self.last_cycle
    }
}

#[derive(Default)]
pub struct Explorer {
    alive: bool,
    current_planet: usize,
}

impl Explorer {
    pub fn new(current_planet: usize) -> Self {
        Self {
            alive: true,
            current_planet,
        }
    }
    
    pub fn kill(&mut self) {
        self.alive = false;
    }
    
    pub fn travel_to_planet(&mut self, destination_planet_index: usize) {
        self.current_planet = destination_planet_index;
    }
    
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    
    pub fn get_current_planet_index(&self) -> usize {
        self.current_planet
    }
    
}