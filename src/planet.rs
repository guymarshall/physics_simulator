use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Planet {
    pub(crate) radius: f32,
    pub(crate) speed: f32,
    pub(crate) angle: f32,
}
