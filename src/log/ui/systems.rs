use crate::log::resources::{LogLevel, LogStore};
use crate::log::ui::components::*;
use crate::log::ui::resources::{DisplayedLogEntries, LogAutoScroll, PreviousPosition};
use crate::log::ui::utils::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub fn spawn_log_ui(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font = asset_server.load(LOG_FONT_PATH);

    commands
        .spawn((
            LogUI,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(LOG_RIGHT_OFFSET), // Try Percent
                top: Val::Px(LOG_TOP_OFFSET), // Try Percent
                bottom: Val::Px(0.0),
                border: UiRect::all(Val::Px(LOG_BORDER_WIDTH)),
                padding: UiRect::all(Val::Px(LOG_PADDING)),
                justify_content: JustifyContent::Center,
                width: Val::Px(LOG_WIDTH),
                height: Val::Px(LOG_HEIGHT),
                ..default()
            },
            BackgroundColor(LOG_BACKGROUND),
            BorderColor::all(LOG_BORDER_COLOR),
            BorderRadius::all(Val::Px(LOG_BORDER_RADIUS)),
        ))
        .with_children(|parent| {
            parent.spawn((
                LogUI,
                Text::new("Log"),
                TextFont {
                    font: font.clone(),
                    font_size: LOG_HEADER_SIZE,
                    ..default()
                },
                TextColor(LOG_TEXT_COLOR),
            ));

            parent
                .spawn((
                    LogUI,
                    LogScrollArea,
                    Node {
                        flex_grow: 1.0,
                        width: Val::Percent(100.0),
                        overflow: Overflow::scroll_y(),
                        flex_direction: FlexDirection::Column,
                        margin: UiRect {
                            top: Val::Px(25.0),
                            bottom: Val::Px(25.0),
                            ..default()
                        },
                        padding: UiRect {
                            bottom: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    },
                    ScrollPosition::default(),
                ))
                .with_children(|scroll| {
                    scroll.spawn((
                        LogUI,
                        LogMessagesContainer,
                        Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(LOG_ROW_GAP),
                            margin: UiRect {
                                bottom: Val::Px(25.0),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

pub fn update_log_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    log_store: Res<LogStore>,
    mut displayed: ResMut<DisplayedLogEntries>,
    container_query: Query<Entity, With<LogMessagesContainer>>,
    mut scroll_query: Query<&mut ScrollPosition, With<LogScrollArea>>, // to update scroll position
    auto_scroll: Res<LogAutoScroll>,
) {
    // If I'm showing all the log messages, I don't need to update
    let total_entries = log_store.len();
    if total_entries <= displayed.count {
        return;
    }

    let Ok(container) = container_query.single() else {
        return;
    };

    let font = asset_server.load(LOG_FONT_PATH);

    let new_entries_count = total_entries - displayed.count;

    let entries: Vec<_> = log_store
        .iter_newest_first()
        .take(new_entries_count)
        .collect();

    commands.entity(container).with_children(|parent| {
        for entry in entries.iter().rev() {
            let color = match entry.level {
                LogLevel::Info => Color::WHITE,
                LogLevel::Warning => Color::srgb(1.0, 0.8, 0.2),
                LogLevel::Error => Color::srgb(1.0, 0.2, 0.2),
            };

            let text = format!("[{:.1}] {}", entry.timestamp, entry.message,);

            parent.spawn((
                LogUI,
                Text::new(text),
                TextFont {
                    font: font.clone(),
                    font_size: LOG_FONT_SIZE,
                    ..default()
                },
                TextColor(color),
            ));
        }
    });

    displayed.count = total_entries;

    // If autoscroll is active, we scroll the log and show the new messages
    if auto_scroll.enabled {
        if let Ok(mut scroll) = scroll_query.single_mut() {
                scroll.y += f32::MAX; // forces to go as down as possible
        }
    }
}

pub fn scroll_log_ui(
    mut wheel_events: MessageReader<MouseWheel>,
    mut scroll_query: Query<&mut ScrollPosition, With<LogScrollArea>>,
    mut auto_scroll: ResMut<LogAutoScroll>,
    mut previous_position: ResMut<PreviousPosition>,
    displayed: Res<DisplayedLogEntries>,
) {
    let Ok(mut scroll) = scroll_query.single_mut() else {
        return;
    };

    if !wheel_events.is_empty() {
        auto_scroll.enabled = false; // User is scrolling
    }

    for event in wheel_events.read() {
        let delta = match event.unit {
            MouseScrollUnit::Line => event.y * 45.0,
            MouseScrollUnit::Pixel => event.y, // Some trackpads send pixels instead of lines
        };
        scroll.y -= delta;
    }

    let user_scrolling_up = scroll.y < previous_position.previous;
    let user_scrolling_down = scroll.y >= previous_position.previous + 10.0;
                                                // +10 is to avoid that micro adjustments take
                                                // the user to the bottom of the log
    if user_scrolling_up {
        auto_scroll.enabled = false;
    } else if user_scrolling_down { // when the user scrolls down a bit (10.0), I take him to the last message
        scroll.y = displayed.count as f32;
        auto_scroll.enabled = true;
    }
    previous_position.previous = scroll.y; // Update previous scroll position
}

pub fn cleanup_log_ui(
    mut commands: Commands,
    query: Query<Entity, With<LogUI>>,
    mut displayed: ResMut<DisplayedLogEntries>,
) {
    displayed.count = 0;
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
