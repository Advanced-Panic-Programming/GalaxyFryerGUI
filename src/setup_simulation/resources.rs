use std::collections::HashSet;
use bevy::prelude::*;
use std::result::Result;
use std::thread::current;
use common_game::components::planet::DummyPlanetState;
use common_game::components::resource::{BasicResource, BasicResourceType, ComplexResource, ComplexResourceType};
use common_game::utils::ID;
use galaxy_fryer::app::gui_protocol::OrchestratorToGUI;
use galaxy_fryer::explorer::bag::{Bag, BagView};

// Galaxy
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
pub struct PlanetsSpritesData {
    pub planets: Vec<PlanetSpriteInfo>,
}

// Struct with the planet sprite info
pub struct PlanetSpriteInfo {
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

// Planet Resource with the actual planet data inside
#[derive(Resource)]
pub struct PlanetsData {
    pub planets: Vec<PlanetInfo>,
}
#[derive(Resource)]
pub struct PlanetInfo {
    id: ID,
    alive: bool,
    energy_cells: Vec<bool>,
    charged_cells_count: usize,
    can_have_rocket: bool,
    has_rocket: bool,
    generate: HashSet<BasicResourceType>,
    combine: HashSet<ComplexResourceType>,
}

impl PlanetInfo {
    // new
    pub fn new(id: ID, alive: bool, energy_cells: Vec<bool>, charged_cells_count: usize, can_have_rocket: bool, has_rocket: bool, generate: HashSet<BasicResourceType>, combine: HashSet<ComplexResourceType>,) -> PlanetInfo {
        PlanetInfo {
            id,
            alive,
            energy_cells,
            charged_cells_count,
            can_have_rocket,
            has_rocket,
            generate,
            combine,
        }
    }

    // Getters
    pub fn get_id(&self) -> ID { self.id }
    pub fn get_alive(&self) -> bool { self.alive }
    pub fn get_charged_energy_cells_count(&self) -> usize { self.charged_cells_count }
    pub fn can_have_rocket(&self) -> bool { self.can_have_rocket }
    pub fn get_rocket(&self) -> bool { self.has_rocket }
    pub fn get_generate(&self) -> &HashSet<BasicResourceType> { &self.generate }
    pub fn get_combine(&self) -> &HashSet<ComplexResourceType> { &self.combine }

    // Setters
    pub fn set_id(&mut self, id: ID) { self.id = id; }
    pub fn kill(&mut self) { self.alive = false; }
    pub fn set_energy_cells(&mut self, energy_cells: Vec<bool>) { self.energy_cells = energy_cells; }
    pub fn set_charged_cells_count(&mut self, charged_cells_count: usize) { self.charged_cells_count = charged_cells_count; }
    pub fn set_can_have_rocket(&mut self, can_have_rocket: bool) { self.can_have_rocket = can_have_rocket; }
    pub fn set_has_rocket(&mut self, rocket: bool) { self.has_rocket = rocket; }
    pub fn set_generate(&mut self, generate: HashSet<BasicResourceType>) { self.generate = generate; }
    pub fn set_combine(&mut self, combine: HashSet<ComplexResourceType>) { self.combine = combine; }
}

// Explorers
#[derive(Resource)]
pub struct ExplorersData {
    pub explorer1: Explorer,
    pub explorer2: Explorer,
    last_cycle: bool,
}

impl ExplorersData {
    pub fn new() -> ExplorersData {
        ExplorersData {
            explorer1: Explorer::new(1, BagView::new()),
            explorer2: Explorer::new(1, BagView::new()),
            last_cycle: false,
        }
    }
    // Setters
    pub fn switch_last_cycle(&mut self) {
        self.last_cycle = !self.last_cycle;
    }
    
    // Getters
    pub fn get_last_cycle(&self) -> bool {
        self.last_cycle
    }
}

pub struct Explorer {
    alive: bool,
    current_planet_index: usize,
    bag: BagView,
}

impl Explorer {
    // New
    pub fn new(current_planet_index: usize, bag: BagView) -> Explorer {
        Explorer {
            alive: true,
            current_planet_index,
            bag,
        }
    }
    
    // Getters
    pub fn is_alive(&self) -> bool { self.alive }
    pub fn get_current_planet_index(&self) -> usize { self.current_planet_index }
    pub fn get_bag(&self) -> &BagView { &self.bag }
    
    // Setters
    pub fn kill(&mut self) {
        self.alive = false;
    }
    pub fn set_current_planet_index(&mut self, index: usize) { self.current_planet_index = index; }
    pub fn set_bag(&mut self, bag: BagView) { self.bag = bag; }
    
}