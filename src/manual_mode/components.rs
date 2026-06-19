use bevy::prelude::*;
use crate::manual_mode::resources::ManualModeTab;

/// Root entity of the bottom manual-mode panel.
#[derive(Component)]
pub struct ManualModePanel;

/// Wraps a single tab's content area; shown/hidden by the active tab.
#[derive(Component)]
pub struct TabContent(pub ManualModeTab);

/// Marks a tab-selector button in the tab bar.
#[derive(Component)]
pub struct TabSelectorBtn(pub ManualModeTab);

// ── Galassia tab ─────────────────────────────────────────────────────────────

#[derive(Component)] pub struct GalaxyTargetDec;
#[derive(Component)] pub struct GalaxyTargetInc;
#[derive(Component)] pub struct GalaxyTargetLabel;
#[derive(Component)] pub struct SendSunrayBtn;
#[derive(Component)] pub struct SendAsteroidBtn;

// ── Explorer tabs (explorer_id = 1 | 2) ─────────────────────────────────────

// Info row
#[derive(Component)] pub struct AskCurrentPlanetBtn(pub u32);
#[derive(Component)] pub struct CurrentPlanetLabel(pub u32);
#[derive(Component)] pub struct AskEnergyCellsBtn(pub u32);
#[derive(Component)] pub struct EnergyCellsLabel(pub u32);

// Bag row
#[derive(Component)] pub struct RefreshBagBtn(pub u32);
#[derive(Component)] pub struct BagLabel(pub u32);

// Move section
#[derive(Component)] pub struct MoveTargetDec(pub u32);
#[derive(Component)] pub struct MoveTargetInc(pub u32);
#[derive(Component)] pub struct MoveTargetLabel(pub u32);
#[derive(Component)] pub struct MoveBtn(pub u32);

// Generate-resource section
#[derive(Component)] pub struct AskSupportedResourcesBtn(pub u32);
#[derive(Component)] pub struct PrevResourceBtn(pub u32);
#[derive(Component)] pub struct NextResourceBtn(pub u32);
#[derive(Component)] pub struct SelectedResourceLabel(pub u32);
#[derive(Component)] pub struct GenerateResourceBtn(pub u32);

// Combine-resource section
#[derive(Component)] pub struct AskSupportedCombinationsBtn(pub u32);
#[derive(Component)] pub struct PrevCombinationBtn(pub u32);
#[derive(Component)] pub struct NextCombinationBtn(pub u32);
#[derive(Component)] pub struct SelectedCombinationLabel(pub u32);
#[derive(Component)] pub struct CombineResourceBtn(pub u32);

// ── Other tab ────────────────────────────────────────────────────────────────

#[derive(Component)] pub struct StopExplorerAIBtn(pub u32);
#[derive(Component)] pub struct StartExplorerAIBtn(pub u32);