use bevy ::prelude::*;

#[derive(Resource, Default)]
pub struct CurrentMode { // Updated in input_handler
    pub current: Mode
}
#[derive(Default, PartialEq)]
pub enum Mode {
    #[default]
    Manual,
    Automatic,
}