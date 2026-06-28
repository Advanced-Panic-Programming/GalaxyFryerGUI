use crate::galaxy_view::components::AnimationConfig;
use crate::planet_view::builders::*;
use crate::planet_view::components::{CornerPlanet, SpawnedByPlanetView};
use crate::planet_view::utils::*;
use crate::setup_simulation::resources::{
    EnergyCellsSpritesData, ExplorerSpriteData, ExplorersData, PlanetTerrainSpriteData,
    PlanetsData, PlanetsSpritesData, RocketSpritesData, SelectedPlanet,
};
use bevy::prelude::*;
use common_game::utils::ID;

// =====================
// === Setup Systems ===
// =====================

/// Spawns the animated corner-planet sprite in the top-right of the screen.
///
/// The planet sprite sheet is a 1-row atlas with 144 frames of 72×72 px each.
/// PLANET_X / PLANET_Y / PLANET_INITIAL_SPLAT are defined in utils.rs.
///
/// Must run after `cleanup` so no stale CornerPlanet entity lingers.
pub fn spawn_corner_planet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    planets_sprites_data: Res<PlanetsSpritesData>,
    selected_planet: Res<SelectedPlanet>,
) {
    let Some(index) = selected_planet.get() else {
        // No planet selected — nothing to show in the corner.
        warn!("spawn_corner_planet: SelectedPlanet has no value, skipping");
        return;
    };

    let planet_to_show = &planets_sprites_data.planets[index];

    let sprite_path = if planet_to_show.alive {
        &planet_to_show.alive_sprite_path
    } else {
        &planet_to_show.destroyed_sprite_path
    };

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(72), 144, 1, None, None);
    let texture_atlas_layout = layouts.add(layout);

    commands.spawn((
        Sprite {
            image: asset_server.load(sprite_path),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(PLANET_X, PLANET_Y, -50.0))
            .with_scale(Vec3::splat(PLANET_INITIAL_SPLAT)),
        AnimationConfig::new(0, 143, ANIMATION_FPS),
        SpawnedByPlanetView,
        CornerPlanet {
            planet_id: index as ID,
        },
    ));
}

/// Spawns the full planet view: terrain, rocket, energy cells and any explorers present.
///
/// The root entity carries only `SpawnedByPlanetView` (used as a despawn anchor) plus the
/// visibility/transform bundle so that Bevy propagates Visibility down to all children.
/// Without `Visibility` + `InheritedVisibility` on the root, child sprites are invisible.
pub fn spawn_planet_view_ui(
    selected: Res<SelectedPlanet>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    planets_data: Res<PlanetsData>,
    planet_terrain_sprite_data: Res<PlanetTerrainSpriteData>,
    explorers_data: Res<ExplorersData>,
    explorer_sprite_data: Res<ExplorerSpriteData>,
    rocket_sprites_data: Res<RocketSpritesData>,
    energy_cells_sprites_data: Res<EnergyCellsSpritesData>,
) {
    let Some(planet_index) = selected.get() else {
        warn!("spawn_planet_view_ui: SelectedPlanet has no value, skipping");
        return;
    };

    let font = asset_server.load(EXPLORERS_BAG_FONT_PATH);

    // Which explorers (if any) are currently on this planet?
    let explorer1_bag = (explorers_data.explorer1.get_current_planet_index() == planet_index)
        .then(|| explorers_data.explorer1.get_bag());
    let explorer2_bag = (explorers_data.explorer2.get_current_planet_index() == planet_index)
        .then(|| explorers_data.explorer2.get_bag());

    // Root entity — invisible spatial anchor used only for grouped despawning.
    //
    // Bevy requires a Visibility component on every ancestor in a hierarchy for
    // InheritedVisibility to propagate correctly to children. Without it, all
    // child sprites render as invisible even though their own Visibility is default.
    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            // Bevy 0.15+ auto-inserts InheritedVisibility and ViewVisibility as
            // required components of Visibility, so we don't need to add them manually.
            SpawnedByPlanetView,
        ))
        .with_children(|child| {
            spawn_terrain(
                child,
                &planets_data.planets[planet_index],
                &planet_terrain_sprite_data.terrain,
                &rocket_sprites_data,
                &energy_cells_sprites_data,
            );

            spawn_explorers(
                child,
                &explorers_data,
                &explorer_sprite_data,
                explorer1_bag,
                explorer2_bag,
                font,
            );
        });
}

// ======================
// === Update Systems ===
// ======================

/// Advances the corner-planet sprite-sheet animation every frame.
pub fn animate_corner_planet(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<CornerPlanet>>,
) {
    match query.single_mut() {
        Ok((mut config, mut sprite)) => {
            config.frame_timer.tick(time.delta());

            if config.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index >= config.last_sprite_index {
                        atlas.index = config.first_sprite_index;
                    } else {
                        atlas.index += 1;
                    }
                }
            }
        }
        Err(e) => warn!("animate_corner_planet: {e}"),
    }
}

/// Re-spawns the entire planet view whenever `SelectedPlanet` changes while already in
/// PlanetView state. Lives in `Update` so its `is_changed` guard is evaluated every frame
/// (unlike `OnEnter` where Bevy silently ignores run conditions).
pub fn respawn_on_planet_change(
    selected: Res<SelectedPlanet>,
    asset_server: Res<AssetServer>,
    planets_data: Res<PlanetsData>,
    planet_terrain_sprite_data: Res<PlanetTerrainSpriteData>,
    explorers_data: Res<ExplorersData>,
    explorer_sprite_data: Res<ExplorerSpriteData>,
    rocket_sprites_data: Res<RocketSpritesData>,
    energy_cells_sprites_data: Res<EnergyCellsSpritesData>,
    existing: Query<Entity, With<SpawnedByPlanetView>>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    planets_sprites_data: Res<PlanetsSpritesData>,
) {
    if !selected.is_changed() {
        return;
    }

    // Despawn the entire previous hierarchy before rebuilding.
    for entity in existing.iter() {
        commands.entity(entity).despawn();
    }

    // Re-run both setup systems inline by duplicating the spawn logic.
    // (Calling the system functions directly is not possible in Bevy's system model;
    //  the cleanest alternative without a full refactor is to forward to the same builders.)
    let Some(planet_index) = selected.get() else {
        return;
    };

    // Corner planet
    let planet_to_show = &planets_sprites_data.planets[planet_index];
    let sprite_path = if planet_to_show.alive {
        &planet_to_show.alive_sprite_path
    } else {
        &planet_to_show.destroyed_sprite_path
    };
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(72), 144, 1, None, None);
    let atlas_layout = layouts.add(layout);

    commands.spawn((
        Sprite {
            image: asset_server.load(sprite_path),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout,
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(PLANET_X, PLANET_Y, -50.0))
            .with_scale(Vec3::splat(PLANET_INITIAL_SPLAT)),
        AnimationConfig::new(0, 143, ANIMATION_FPS),
        SpawnedByPlanetView,
        CornerPlanet {
            planet_id: planet_index as ID,
        },
    ));

    // Main view
    let font = asset_server.load(EXPLORERS_BAG_FONT_PATH);
    let explorer1_bag = (explorers_data.explorer1.get_current_planet_index() == planet_index)
        .then(|| explorers_data.explorer1.get_bag());
    let explorer2_bag = (explorers_data.explorer2.get_current_planet_index() == planet_index)
        .then(|| explorers_data.explorer2.get_bag());

    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            SpawnedByPlanetView,
        ))
        .with_children(|child| {
            spawn_terrain(
                child,
                &planets_data.planets[planet_index],
                &planet_terrain_sprite_data.terrain,
                &rocket_sprites_data,
                &energy_cells_sprites_data,
            );
            spawn_explorers(
                child,
                &explorers_data,
                &explorer_sprite_data,
                explorer1_bag,
                explorer2_bag,
                font,
            );
        });
}

pub fn update_current_planet() {
    // TODO!
}

// ======================
// === OnExit Systems ===
// ======================

/// Despawns all entities tagged `SpawnedByPlanetView`, including their full child hierarchy.
/// `despawn_related::<ChildOf>` removes all children recursively before `despawn` removes
/// the root — leaving no orphaned entities in the world.
pub fn cleanup(mut commands: Commands, query: Query<Entity, With<SpawnedByPlanetView>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}