use bevy::{prelude::*, render::camera::Camera, transform::TransformSystem};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum FseNormalizeSystem {
    Normalize,
}

pub struct Ui3dNormalization;
impl Plugin for Ui3dNormalization {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            normalize
                .system()
                .label(FseNormalizeSystem::Normalize)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

/// Marker struct that marks entities with meshes that should be scaled relative to the camera.
pub struct Normalize3d;

#[allow(clippy::type_complexity)]
pub fn normalize(
    camera_query: Query<&GlobalTransform, With<Camera>>,
    mut normalize_query: Query<&mut Transform, With<Normalize3d>>,
) {
    // TODO: can be improved by manually specifying the active camera to normalize against. The
    // majority of cases will only use a single camera for this viewer, so this is sufficient.
    let camera_position = if let Some(pos) = camera_query.iter().next() {
        pos.clone()
    } else {
        error!("failed to find camera");
        return;
    };
    for mut transform in normalize_query.iter_mut() {
        let distance = -camera_position
            .compute_matrix()
            .inverse()
            .transform_point3(transform.translation)
            .z;

        transform.scale = Vec3::splat(distance / 12.0);
    }
}
