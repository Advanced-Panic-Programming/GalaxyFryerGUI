use bevy::prelude::*;

#[derive(Bundle)]
pub (super) struct LegendTextBundle {
    pub text: Text,
    pub text_font: TextFont,
    pub text_color: TextColor,
}

#[derive(Bundle)]
pub (super) struct KeyCapBundle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub border_color: BorderColor,
    pub border_radius: BorderRadius,
}