use bevy::prelude::*;
use common_game::utils::ID;
use crate::manual_mode::resources::Tab;

// ManualMode Panel Hierarchy
// ManualModePanelRoot
// │
// ├── TabBar
// │   ├── TabButton(Galaxy)
// │   ├── TabButton(Explorer1)
// │   └── TabButton(Explorer2)
// │
// └── ContentContainer
// ├── TabContent(Galaxy)
// ├── TabContent(Explorer1)
// └── TabContent(Explorer2)

/// Root entity of the manual mode panel
#[derive(Component)]
pub struct ManualModePanelRoot;

#[derive(Component)]
pub struct TabButton {
    pub tab: Tab,
}

#[derive(Component)]
pub struct TabContent {
    pub tab: Tab,
}

// ==========================
//      Spinners Buttons
// ==========================
#[derive(Component)]
pub struct PlanetSpinnerValue; // Used to update the spinner value

#[derive(Component)]
pub struct PlanetSpinnerDecrementButton;

#[derive(Component)]
pub struct PlanetSpinnerIncrementButton;

// Same scheme as the planet spinner above applied to all the other spinners below
#[derive(Component)]
pub struct GenerateResourceSpinnerValue {
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct GenerateResourceSpinnerDecrementButton{
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct GenerateResourceSpinnerIncrementButton{
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct CombineResourceSpinnerValue{
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct CombinableResourceSpinnerDecrementButton{
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct CombinableResourceSpinnerIncrementButton{
    pub explorer_id: ID,
}

// ==========================
//      Galaxy Tab Buttons
// ==========================
#[derive(Component)]
pub struct SendSunrayButton;

#[derive(Component)]
pub struct SendAsteroidButton;

// ====================================================
//        Explorer1/2 Tab Buttons and Markers
// ====================================================
#[derive(Component)]
pub struct MoveButton {
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct AskGenerateButton {
    pub explorer_id: ID,    
}

#[derive(Component)]
pub struct GenerateButton {
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct AskCombineButton {
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct CombineButton {
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct StartAIButton{
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct StopAIButton{
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct ExplorerCurrentPlanetMarker{
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct BagViewMarker{
    pub explorer_id: ID,
}

#[derive(Component)]
pub struct ExplorerSpriteMarker{
    pub explorer_id: ID,
}
