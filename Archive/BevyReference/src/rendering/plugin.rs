use bevy::prelude::*;

use super::camera;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        // Dark backdrop so any transparent regions in title/menu art (e.g. the
        // title's menu cutout) read as near-black rather than the default gray.
        app.insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.03)))
            .add_systems(Startup, camera::spawn_primary_camera);
    }
}
