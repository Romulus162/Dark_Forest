use crate::{
    // file_system_interaction::audio::AudioHandles,
    // movement::character_controller::*,
    player_control::{
        actions::{DualAxisDataExt, PlayerAction},
        // camera::{CameraUpdateSystemSet, IngameCamera, IngameCameraKind},
    },
};

use crate::{
    // level_instantiation::on_spawn::Player,util::math_trait_ext::Vec3Ext,
    // world_interaction::dialog::CurrentDialogTarget, GameState
};
use anyhow::Context;
use bevy::prelude::*;
// use bevy_kira_audio::AudioInstance;
use bevy_mod_sysfail::prelude::*;
// use bevy_tnua::{builtins::TnuaBuiltinWalk, controller::TnuaController};
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::ActionState;

/// This plugin handles everything that has to do with the player's physical representation in the world.
/// This includes movement and rotation that differ from the way the [`crate::movement::plugin`] already handles characters in general.
pub(super) fn plugin(app: &mut App) {
    // app.register_type::<Timer>()
        // .register_type::<Player>()
        // .add_systems(
                    // Update,
                    // (
                        // handle_jump,
                        // handle_horizontal_movement,
                        // control_walking_sound,
                        // rotate_to_speaker,
                        // handle_camera_kind,
                    // )
                        // .chain()
                        // .before(CameraUpdateSystemSet)
                        // .before(GeneralMovementSystemSet)
                        // .after(InputManagerSystem::ManualControl)
                        // .run_if(in_state(GameState::Playing)),
        // );
}

// fn handle_jump(mut player_query: Query<(&ActionState<PlayerAction>, &mut Jump), With<Player>>) {
//     #[cfg(feature = "tracing")]
//     let _span = info_span!("handle_jump").entered();
//     for (actions, mut jump) in &mut player_query {
//         jump.requested
//     }
// }

// #[sysfail(Log<anyhow::Error, Error>)]
// fn handle_horizontal_movement(
//     mut player_query: Query<(&ActionState<PlayerAction>, &mut Walk, &mut Sprinting), With<Player>>,
//     camera_query: Query<(&IngameCamera, &Transform), Without<Player>>,
// ) {

// }
///////////// A LOT OF STUFF STILL NEEDED TO BE DONE HERE
