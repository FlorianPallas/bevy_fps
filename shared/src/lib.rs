use bevy::prelude::*;

pub mod consts;
pub mod interpolate;
pub mod pawns;
pub mod plugins;
pub mod scenes;

#[derive(Debug, Clone, Default)]
pub struct Command {
    pub angle: Vec2,
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub jump: bool,
    pub sneak: bool,
    pub crouch: bool,
    pub fire: bool,
}
