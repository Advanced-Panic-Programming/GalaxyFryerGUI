use common_game::utils::ID;
use crate::log::resources::*;
use crate::log::resources::LogLevel::*;

impl LogMessage {
    // Planet
    pub fn planet_destroyed(p_id: ID) -> Self {
        Self {
            level: Fatal,
            message: format!("Planet #{} was destroyed", p_id+1),
        }
    }

    pub fn asteroid_destroyed(p_id: ID) -> Self {
        Self {
            level: Basic,
            message: format!("Planet #{} | destroyed incoming asteroid", p_id+1),
        }
    }

    // Explorer
    pub fn explorer_moved(explorer_id: ID, planet_id: ID) -> Self {
        Self {
            level: Basic,
            message: format!("Explorer #{} | moved to planet #{}", explorer_id+1, planet_id+1),
        }
    }

    pub fn explorer_killed(explorer_id: ID) -> Self {
        Self {
            level: Fatal,
            message: format!("Explorer #{} | killed", explorer_id+1),
        }
    }
    
    pub fn explorer_state( explorer_id: ID, explorer_state: String) -> Self {
        Self {
            level: Basic,
            message: format!("Explorer #{} is in state [{}]", explorer_id + 1, explorer_state),
        }
    }

    // Simulation
    pub fn automatic_mode_ack() -> Self {
        Self {
            level: Info,
            message: "Simulation | automatic mode activated".into(),
        }
    }

    pub fn send_sunray_ack(planet_id: ID) -> Self {
        Self {
            level: Info,
            message: format!("Sunray sent to planet #{}", planet_id + 1),
        }
    }

    pub fn send_asteroid_ack(planet_id: ID) -> Self {
        Self {
            level: Info,
            message: format!("Asteroid sent to planet #{}", planet_id + 1),
        }
    }

    pub fn manual_mode_ack() -> Self {
        Self {
            level: Info,
            message: "Simulation | manual mode activated".into(),
        }
    }

    pub fn simulation_end() -> Self {
        Self {
            level: Info,
            message: "Simulation | ended".into(),
        }
    }
}