use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);

pub const DELTA_TIME: f32 = 1. / 60.;

/// Moves objects in the physics world
pub fn simulate(mut query: Query<(&mut Pos, &mut PrevPos)>) {
    for (mut pos, mut prev_pos) in query.iter_mut() {
        let velocity = (pos.0 - prev_pos.0) / DELTA_TIME;
        prev_pos.0 = pos.0;
        pos.0 += velocity * DELTA_TIME;
    }
}

/// Copies positions from the physics world to bevy Transforms
pub fn sync_transforms(mut query: Query<(&mut bevy::transform::components::Transform, &Pos)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
    }
}
