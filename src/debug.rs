use bevy::{app::Plugin, ecs::component::Component, prelude::*};

use crate::gameplay::AABB;

#[derive(Resource)]
struct DebugEnabled(bool);

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, (draw_bounding_boxes, toggle_debug));
        app.insert_resource(DebugEnabled(false));
    }
}

fn draw_bounding_boxes(
    query: Query<(&AABB, &GlobalTransform)>,
    mut gizmos: Gizmos,
    debug_enabled: Res<DebugEnabled>,
) {
    if !debug_enabled.0 {
        return;
    }
    for (aabb, trans) in query.iter() {
        let rect = aabb.rect.translate(trans.translation().xy());
        gizmos.rect_2d(
            Isometry2d::from_translation(rect.center()),
            rect.size(),
            LinearRgba::RED,
        );
    }
}

fn toggle_debug(input: Res<ButtonInput<KeyCode>>, mut dbg: ResMut<DebugEnabled>) {
    if input.just_pressed(KeyCode::KeyD) {
        dbg.0 = !dbg.0;
    }
}
