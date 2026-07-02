use bevy::prelude::*;
use common_game::utils::ID;

// ── Despawn anchor ────────────────────────────────────────────────────────────

/// Marks every entity owned by PlanetView.
/// A single query on this component is enough to despawn the whole view.
#[derive(Component)]
pub struct SpawnedByPlanetView;

// ── Corner planet (top-right animated sprite) ─────────────────────────────────

/// The animated planet sprite shown in the corner of the screen.
#[derive(Component)]
pub struct CornerPlanet;

// ── Terrain / background ──────────────────────────────────────────────────────

/// The full-screen terrain background sprite.
/// Carries no extra data — updated by swapping the image handle directly.
#[derive(Component)]
pub struct TerrainBackground;

// ── Rocket ────────────────────────────────────────────────────────────────────

/// The rocket sprite. `planet_id` lets update systems find it without ambiguity.
#[derive(Component)]
pub struct Rocket;

// ── Energy cells ──────────────────────────────────────────────────────────────

/// One energy-cell sprite. `cell_index` identifies its position in the row so
/// update systems can patch individual cells without rebuilding the whole row.
#[derive(Component)]
pub struct EnergyCell {
    pub cell_index: usize,
}

// ── Explorers ─────────────────────────────────────────────────────────────────

/// The explorer body sprite.
#[derive(Component)]
pub struct ExplorerSprite {
    pub explorer_id: ID,
}

/// The explorer's bag label (Text 2d).
#[derive(Component)]
pub struct ExplorerBagLabel {
    pub explorer_id: ID,
}