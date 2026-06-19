use bevy::prelude::*;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};

/// Which tab of the manual-mode panel is currently shown.
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum ManualModeTab {
    #[default]
    Galaxy,
    Explorer1,
    Explorer2,
    Other,
}

/// Cached display state for one explorer's panel column.
pub struct ExplorerPanelState {
    /// Display string for the current planet (updated after AskCurrentPlanet).
    pub current_planet: String,
    /// Display string for energy cells (updated after AskEnergyCells).
    pub energy_cells: String,
    /// Display string for bag contents (updated after BagContentRequest).
    pub bag: String,
    /// Populated after SupportedResourceRequest is answered.
    pub supported_resources: Vec<BasicResourceType>,
    pub resource_cursor: usize,
    /// Populated after SupportedCombinationRequest is answered.
    pub supported_combinations: Vec<ComplexResourceType>,
    pub combination_cursor: usize,
    /// Planet ID the player has dialled in to move to (1-7).
    pub move_target: u32,
}

impl Default for ExplorerPanelState {
    fn default() -> Self {
        Self {
            current_planet: "?".into(),
            energy_cells: "?".into(),
            bag: "(empty)".into(),
            supported_resources: Vec::new(),
            resource_cursor: 0,
            supported_combinations: Vec::new(),
            combination_cursor: 0,
            move_target: 1,
        }
    }
}

impl ExplorerPanelState {
    pub fn selected_resource_name(&self) -> String {
        if self.supported_resources.is_empty() {
            return "—  (to ask)".into();
        }
        format!("{:?}", self.supported_resources[self.resource_cursor])
    }

    pub fn selected_combination_name(&self) -> String {
        if self.supported_combinations.is_empty() {
            return "—  (to ask)".into();
        }
        format!("{:?}", self.supported_combinations[self.combination_cursor])
    }
}

/// Drives the entire manual-mode bottom panel.
#[derive(Resource)]
pub struct ManualModeState {
    pub active: bool,
    pub active_tab: ManualModeTab,
    pub explorer1: ExplorerPanelState,
    pub explorer2: ExplorerPanelState,
    /// Planet ID targeted in the Galaxy tab (1-7).
    pub galaxy_target: u32,
}

impl Default for ManualModeState {
    fn default() -> Self {
        Self {
            active: true, // simulation starts in manual mode
            active_tab: ManualModeTab::default(),
            explorer1: ExplorerPanelState::default(),
            explorer2: ExplorerPanelState::default(),
            galaxy_target: 0,
        }
    }
}

impl ManualModeState {
    pub fn explorer(&self, id: u32) -> &ExplorerPanelState {
        if id == 1 { &self.explorer1 } else { &self.explorer2 }
    }
    pub fn explorer_mut(&mut self, id: u32) -> &mut ExplorerPanelState {
        if id == 1 { &mut self.explorer1 } else { &mut self.explorer2 }
    }
}