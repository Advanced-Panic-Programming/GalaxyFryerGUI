use std::collections::HashSet;
use std::fmt;
use common_game::utils::ID;
use crate::log::resources::*;
use crate::log::resources::LogLevel::*;

impl LogMessage {
    // Planet
    pub fn planet_destroyed(p_id: ID) -> Self {
        Self {
            level: Error,
            message: format!("Planet #{p_id} was destroyed"),
        }
    }

    pub fn asteroid_destroyed(p_id: ID) -> Self {
        Self {
            level: Info,
            message: format!("Planet #{p_id} | destroyed incoming asteroid"),
        }
    }

    // Explorer
    pub fn explorer_moved(explorer_id: ID, planet_id: ID) -> Self {
        Self {
            level: Info,
            message: format!("Explorer #{explorer_id} | moved to planet #{planet_id}"),
        }
    }

    pub fn explorer_killed(explorer_id: ID) -> Self {
        Self {
            level: Error,
            message: format!("Explorer #{explorer_id} | killed"),
        }
    }

    // Simulation
    pub fn automatic_mode_ack() -> Self {
        Self {
            level: Warning,
            message: "Simulation | automatic mode activated".into(),
        }
    }

    pub fn manual_mode_ack() -> Self {
        Self {
            level: Warning,
            message: "Simulation | manual mode activated".into(),
        }
    }

    pub fn simulation_end() -> Self {
        Self {
            level: Warning,
            message: "Simulation | ended".into(),
        }
    }
}