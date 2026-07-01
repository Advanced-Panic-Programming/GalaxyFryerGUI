use bevy::prelude::*;
use galaxy_fryer::app::gui_protocol::GUIToOrchestrator::AskPlanetState;
use crate::app_state_manager::messages::{PlayPressed, ExitPressed};
use crate::pause_menu::components::*;
use crate::pause_menu::utils::*;
use crate::setup_orchestrator::resources::ToOrchestrator;

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
    // === Tittle =======
    // =========================

    let title_font = asset_server.load(TITLE_FONT);

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(TITLE_TOP_PADDING),
            width: Val::Percent(100.0),

            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            ..default()
        })
        .with_child((
            Text::new("GalaxyFryer"),
            TextFont {
                font: title_font,
                font_size: TITLE_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            TextShadow {
                offset: Vec2::splat(8.0),
                ..default()
            },
            PauseMenuUI,
            AnimatedTitle,
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
        Transform::from_translation(Vec3::new(0.0, 10.0, 10.0))
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

    // ===============================
    // === Play/Exit Buttons =========
    // ===============================

    let buttons_font = asset_server.load(BUTTONS_FONT);

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            padding: UiRect::bottom(Val::Px(48.0)),
            ..default()
        },
        PauseMenuUI,
        children![(
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(BUTTON_SPACING), // Buttons spacing
            ..default()
        },
        children![
                // Play Button
                (PlayButton, create_button(buttons_font.clone(), "PLAY")),
                // Exit Button
                (ExitButton, create_button(buttons_font.clone(), "EXIT")),
            ]
        )],
    ));

}

fn create_button(font: Handle<Font>, label: &str) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(BUTTON_WIDTH),
            height: Val::Px(BUTTON_HEIGHT),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        BorderColor::all(Color::srgb(0.05, 0.05, 0.05)),
        BorderRadius::all(Val::Px(10.0)),
        children![(
            Text::new(label),
            TextFont { font_size: FONT_SIZE, font: font.clone(), ..default() },
            TextColor(Color::WHITE),
        )],
    )
}

pub fn animate_tittle(
    time: Res<Time>,
    mut query: Query<&mut Node, With<AnimatedTitle>>,
) {
    for mut node in &mut query {
        node.top = Val::Px(
            TITLE_TOP_PADDING
                + TITLE_OSCILLATION * ops::cos(time.elapsed_secs()*TITLE_SPEED)
        );
    }
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

        if timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index >= indices.last {
                atlas.index = indices.first;
            } else {
                atlas.index += 1;
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

pub fn exit_button_system(
    interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
    mut writer: MessageWriter<ExitPressed>,
) {
    for (interaction, mut color) in interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.75, 0.35, 0.35));
                // Generates event handled in app_state_manager
                writer.write(ExitPressed);
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

// Needs to be done after the setup_orchestrator module and before the actual simulation start in AppState::GalaxyView
pub fn init_planet_states(
    sender: Res<ToOrchestrator>,
) {
    for planet_id in 0..7 {
        let _ = sender.0.send(AskPlanetState { planet_id });
    }
}