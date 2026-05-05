use std::ops::Index;
use bevy::prelude::*;
use crate::galaxy_view::components::{AnimationConfig, Planet};
use crate::planet_view::components::SpawnedByPlanetView;
use crate::planet_view::utils::*;
use crate::setup_simulation::resources::{ExplorersData, PlanetsData, SelectedPlanet};


// =====================
// === Setup Systems ===
// =====================

pub fn spawn_planet_view_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    
) {
    
}

// ======================
// === Update Systems ===
// ======================

pub fn animate_corner_planet(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Planet>>
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
        Err(error) => {
            println!("{}", error)
        }
    }
}

pub fn refresh_planet_view(
    selected: Res<SelectedPlanet>,
    explorer_data: Res<ExplorersData>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    planet_data: Res<PlanetsData>,
    current_entities: Query<Entity, With<SpawnedByPlanetView>>,
) {
    if !selected.is_changed() {
        return;
    }

    // Despawn old
    for entity in current_entities.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn corner planet
    if let Some(index) = selected.get() {
        let planet_to_show = &planet_data.planets[index];

        let sprite_path = if planet_to_show.alive {
            &planet_to_show.sprite_path
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
            Planet{
                index,
            },
            SpawnedByPlanetView,
        ));

        if planet_to_show.alive {
            // Spawn planet terrain and attributes (rocket, crafting, ...)


            // Spawn explorer
            let mut explorer1_to_draw = None;
            let mut explorer2_to_draw = None;
            if let Some(index) = selected.get() {
                if explorer_data.explorer1.get_current_planet_index() == index {
                    explorer1_to_draw = Some(&explorer_data.explorer1);
                } else {
                    explorer1_to_draw = None;
                }
                if explorer_data.explorer2.get_current_planet_index() == index {
                    explorer2_to_draw = Some(&explorer_data.explorer2);
                } else {
                    explorer2_to_draw = None;
                }
            }

            match (explorer1_to_draw, explorer2_to_draw) {
                (Some(explorer1_data), Some(explorer2_data)) => {

                },
                (Some(explorer1_data), None) => {

                },
                (None, Some(explorer2_data)) => {

                }
                (None, None) => {

                }
            }
        } else {
            // Show dead terrain and planet exploded message

        }
    }




}

// ======================
// === OnExit Systems ===
// ======================
pub fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<SpawnedByPlanetView>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}