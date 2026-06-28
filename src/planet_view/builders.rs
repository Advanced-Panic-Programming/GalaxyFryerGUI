use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use common_game::utils::ID;
use galaxy_fryer::explorer::bag::BagView;

use crate::planet_view::components::*;
use crate::setup_simulation::resources::{
    EnergyCellsSpritesData, ExplorerSpriteData, ExplorerSpriteInfo, ExplorersData, PlanetInfo,
    PlanetTerrainSpriteInfo, RocketSpritesData,
};

// ──────────────────────────────────────────────
//  Layout constants
//  All sizes are in world-space pixels (Camera2d 1:1, 1 unit = 1 px).
// ──────────────────────────────────────────────

/// Rocket sprite is 1024×1024 px — scale it down to fit the terrain backdrop.
const ROCKET_DISPLAY_SIZE: Vec2 = Vec2::new(200.0, 200.0);
const ROCKET_POS: Vec3 = Vec3::new(-680.0, -390.0, 1.0);

/// Energy-cell sprite is 300×300 px — render at 50×50 px so the whole row fits.
const CELL_DISPLAY_SIZE: Vec2 = Vec2::new(50.0, 50.0);
/// Horizontal gap between cell centres (display size + 10 px margin).
const CELLS_SPACING: f32 = 60.0;
const CELLS_START_X: f32 = -500.0;
const CELLS_Y: f32 = -460.0;
const CELLS_Z: f32 = 1.0;

/// Explorer sprite is 288×288 px — render at 160×160 px.
const EXPLORER_DISPLAY_SIZE: Vec2 = Vec2::new(160.0, 160.0);
/// How many pixels above the explorer centre the bag text appears.
const BAG_TEXT_OFFSET_Y: f32 = 100.0;

// ──────────────────────────────────────────────
//  Terrain, rocket, energy cells
// ──────────────────────────────────────────────

/// Spawns the planet terrain background, the rocket (if present) and all energy-cell slots.
/// Every entity is a pure world-space sprite (Transform + Sprite) so they share the same
/// coordinate system as the camera.
pub fn spawn_terrain(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    planet_info: &PlanetInfo,
    planet_terrain_sprite_info: &PlanetTerrainSpriteInfo,
    rocket_sprites_data: &RocketSpritesData,
    energy_cells_sprites_data: &EnergyCellsSpritesData,
) {
    // Background terrain — stretched to fill a 1920×1080 viewport exactly.
    let terrain_image = if planet_info.get_alive() {
        planet_terrain_sprite_info.alive.clone()
    } else {
        planet_terrain_sprite_info.destroyed.clone()
    };

    parent.spawn((
        Sprite {
            image: terrain_image,
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -80.0),
    ));

    // Rocket — only on planets that support it.
    // custom_size overrides the native 1024×1024 texture size.
    if planet_info.can_have_rocket() {
        let rocket_image = if planet_info.get_rocket() {
            rocket_sprites_data.rocket.full_built_rocket.clone()
        } else {
            rocket_sprites_data.rocket.empty_rocket_base.clone()
        };

        parent.spawn((
            Sprite {
                image: rocket_image,
                custom_size: Some(ROCKET_DISPLAY_SIZE),
                ..default()
            },
            Transform::from_translation(ROCKET_POS),
            Rocket {
                planet_id: planet_info.get_id(),
            },
        ));
    }

    // Energy cells — evenly spaced row along the bottom of the screen.
    // custom_size overrides the native 300×300 texture size.
    for (i, &charged) in planet_info.get_energy_cells().iter().enumerate() {
        let image = if charged {
            energy_cells_sprites_data.energy_cell.charged.clone()
        } else {
            energy_cells_sprites_data.energy_cell.empty.clone()
        };

        parent.spawn((
            Sprite {
                image,
                custom_size: Some(CELL_DISPLAY_SIZE),
                ..default()
            },
            Transform::from_xyz(
                CELLS_START_X + i as f32 * CELLS_SPACING,
                CELLS_Y,
                CELLS_Z,
            ),
            EnergyCells {
                planet_id: planet_info.get_id(),
            },
        ));
    }
}

// ──────────────────────────────────────────────
//  Explorers
// ──────────────────────────────────────────────

/// Spawns both explorers onto the planet view.
/// `explorerN_bag = None`  → explorer is not on this planet, nothing is spawned.
/// `explorerN_bag = Some`  → explorer is present; sprite + bag text are spawned.
pub fn spawn_explorers(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    explorer_data: &ExplorersData,
    explorer_sprite_data: &ExplorerSpriteData,
    explorer1_bag: Option<&BagView>,
    explorer2_bag: Option<&BagView>,
    font: Handle<Font>,
) {
    spawn_single_explorer(
        parent,
        &explorer_sprite_data.explorer1,
        explorer_data.explorer1.is_alive(),
        0,
        explorer1_bag,
        Vec3::new(-350.0, -120.0, 75.0),
        font.clone(), // clone so the handle stays alive for explorer 2
    );

    spawn_single_explorer(
        parent,
        &explorer_sprite_data.explorer2,
        explorer_data.explorer2.is_alive(),
        1,
        explorer2_bag,
        Vec3::new(350.0, -120.0, 75.0),
        font,
    );
}

/// Spawns a single explorer sprite and its bag label as world-space entities.
///
/// Two separate child entities are created:
/// * **Sprite entity** — the explorer graphic at `position`.
/// * **Text entity**   — the bag summary `BAG_TEXT_OFFSET_Y` units above the sprite,
///                       using `Text2d` (world-space text, no Node required).
///
/// Nothing is spawned when `option_bag` is `None` (explorer absent from this planet).
fn spawn_single_explorer(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    sprite: &ExplorerSpriteInfo,
    alive: bool,
    explorer_id: ID,
    option_bag: Option<&BagView>,
    position: Vec3,
    font: Handle<Font>,
) {
    // Explorer is not on this planet — skip silently.
    let Some(bag) = option_bag else {
        return;
    };

    let texture = if alive {
        sprite.alive_sprite.clone()
    } else {
        sprite.dead_sprite.clone()
    };

    // Explorer sprite — custom_size overrides the native 288×288 texture.
    parent.spawn((
        Sprite {
            image: texture,
            custom_size: Some(EXPLORER_DISPLAY_SIZE),
            ..default()
        },
        Transform::from_translation(position),
        Explorer { explorer_id },
    ));

    // Bag label in world-space via Text2d.
    //
    // IMPORTANT: Text2d auto-inserts Transform via Bevy's Required Components system.
    // Adding Transform in the same spawn tuple would create a duplicate-component conflict
    // that the compiler reports as "not a Bundle". We spawn without Transform and then
    // immediately override the auto-inserted default via .insert().
    let text_pos = Vec3::new(
        position.x,
        position.y + BAG_TEXT_OFFSET_Y,
        position.z + 1.0, // render in front of the sprite
    );

    parent
        .spawn((
            Text2d::new(format!("{bag}")),
            TextFont::from(font).with_font_size(24.0),
            TextColor(Color::WHITE),
            ExplorerBag { explorer_id },
        ))
        .insert(Transform::from_translation(text_pos));
}