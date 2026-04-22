use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Planet {
    pub(crate) radius: f32,
    pub(crate) speed: f32,
    pub(crate) size: f32,
    pub(crate) angle: f32,
}

impl Planet {
    pub(crate) fn new(radius: f32, speed: f32, size: f32, angle: f32) -> Planet {
        Planet {
            radius,
            speed,
            size,
            angle,
        }
    }
}
