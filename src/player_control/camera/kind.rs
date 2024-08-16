use crate::{
    file_system_interaction::config::GameConfig,
    player_control::{
        actions::CameraAction,
        camera::{IngameCamera, IngameCameraKind},
    },
};
use bevy::prelude::*;
use bevy_dolly::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) fn update_kind(
    mut camera_query: Query<(&mut IngameCamera, &ActionState<CameraAction>)>,
    config: Res<GameConfig>,
) {
    for (mut cameram, actions) in camera_query.iter_mut() {
        let zoom = actions.clamped_value(&CameraAction::Zoom);
        let zoomed_out = zoom < -1e-5;
        let zoomed_in = zoom > 1e-5;
        let new_kind = match camera.kind {
            IngameCameraKind::FirstPerson if zoomed_out => Some(IngameCameraKind::ThirdPerson),
            IngameCameraKind::ThirdPerson
                if camera.desired_distance < config.camera.third_person.min_distance + 1e-5
                    && zoomed_in =>
            {
                Some(IngameCameraKind::FirstPerson)
            }
            IngameCameraKind::ThirdPerson
                if camera.desired_distance > config.camera.third_person.max_distance - 1e-5
                    && zoomed_out =>
            {
                Some(IngameCameraKind::FixedAngle)
            }
            IngameCameraKind::FixedAngle
                if camera.desired_distance < config.camera.fixed_angle.min_distance + 1e-5
                    && zoomed_in =>
            {
                Some(IngameCameraKind::ThirdPerson)
            }
            _ => None,
        };
        if let Some(new_kind) = new_kind {
            camera.kind = new_kind;
        }
    }
}

pub(super) fn update_drivers(mut camera_query: Query<(IngameCamera, &mut Rig)>) {
    for (camera, mut rig) in camera_query.iter_mut() {
        match camera.kind {
            IngameCameraKind::ThirdPerson => set_third_person_drivers(&mut rig),
            IngameCameraKind::FirstPerson => match camera.secondary_target {
                Some(_) => set_first_person_drivers_with_target(&mut rig),
                None => set_first_person_drivers_without_target(&mut rig),
            },
            IngameCameraKind::FixedAngle => set_fixed_angle_drivers(&mut rig),
        };
    }
}

fn set_third_person_drivers(rig: &mut Rig) {
    rig.ensure_driver_exists(Arm::new(default()));
    // Overriding because tracking_predictive cannot be changed after creation.
    rig.override_driver(LookAt::new(default()).tracking_predictive(true));
}
