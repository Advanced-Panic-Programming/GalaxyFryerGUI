use bevy::prelude::*;
use crate::galaxy_view::ui::utils::*;

#[derive(Bundle)]
pub struct BaseBox {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}

impl BaseBox {
    pub fn get_base_box() -> BaseBox {
        BaseBox {
            node: Node {
                width: Val::Percent(35.0),   // % width
                height: Val::Percent(95.0),  // % height
                margin: UiRect {
                    right: Val::Px(RIGHT_MARGIN),
                    top: Val::Px(TOP_MARGIN),
                    bottom: Val::Px(BOTTOM_MARGIN),
                    ..default()
                },
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(), // if text too long
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(BOX_BG_COLOR.into()),
            border_color: BorderColor::all(BOX_BORDER_COLOR),
            border_radius: BorderRadius::all(Val::Px(BOX_BORDER_RADIUS)),
        }
    }
}