use bevy::prelude::*;
use crate::app_state_manager::messages::PlayPressed;
use crate::galaxy_view::components::AnimationConfig;
use crate::galaxy_view::utils::{ANIMATION_FPS, PLANET_INITIAL_SPLAT};
use crate::pause_menu::components::*;
use crate::pause_menu::utils::*;

pub fn setup_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    // =========================
    // === Background UI =======
    // =========================

    let background = asset_server.load(MENU_BACKGROUND);
    commands.spawn((
        Sprite {
        image: background,
        custom_size: Some(Vec2::new(1920.0, 1080.0)),
        ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
        PauseMenuUI
    ));
    
    // =========================
    // === Animated Logo =======
    // =========================

    let texture: Handle<Image> =
        asset_server.load(LOGO_SPRITE_SHEET);

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(LOGO_FRAME_SIZE),
        LOGO_FRAMES,
        LOGO_ROWS,
        None,
        None,
    );

    let atlas_layout = layouts.add(layout);

    commands.spawn((
        PauseMenuUI,
        AnimatedLogo,
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout,
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 120.0, 10.0))
            .with_scale(Vec3::splat(LOGO_SCALE)),
        AnimationIndices {
            first: 0,
            last: (LOGO_FRAMES as usize) - 1,
        },
        AnimationTimer(Timer::from_seconds(
            1.0 / LOGO_FPS,
            TimerMode::Repeating,
        )),
    ));

    // =========================
    // === Play Button =========
    // =========================

    commands
        .spawn((
            PauseMenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PauseMenuUI,
                    PlayButton,
                    Button,
                    Node {
                        width: Val::Px(PLAY_BUTTON_WIDTH),
                        height: Val::Px(PLAY_BUTTON_HEIGHT),

                        // Push button LOWER than center
                        margin: UiRect::top(Val::Px(320.0)),

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(18.0)),
                    BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PLAY"),
                        TextFont {
                            font_size: 42.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn animate_logo(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut Sprite,
        ),
        With<AnimatedLogo>,
    >,
) {
    for (indices, mut timer, mut sprite) in &mut query {

        timer.tick(time.delta());

        if timer.just_finished() {

            if let Some(atlas) = &mut sprite.texture_atlas {

                if atlas.index >= indices.last {
                    atlas.index = indices.first;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}

pub fn play_button_system(
    interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut writer: MessageWriter<PlayPressed>,
) {
    for (interaction, mut color) in interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));
                // Generates event handled in app_state_manager
                writer.write(PlayPressed);
            }

            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }

            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}