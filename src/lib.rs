// These two generate a lot of false positives for Bevy systems
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
// This is not a library, so we don't need to worry about intra-doc links
#![allow(rustdoc::private_intra_doc_links)]


//! Dark Forest is split into many plugins with their own set of responsibilities.
//! This is an organizational measure I have addapted and basing off of the fox_trot framework
//! everyting seen here, is a reverse engineer project of that framework.

use bevy::prelude::*;
mod bevy_config;
#[cfg(feature = "dev")]
mod dev;
mod file_system_interaction;
mod ingame_menu;
mod level_instantiation;
mod menu;
pub(crate) mod movement;
pub(crate) mod particles;
mod player_control;
mod shader;
mod system_set;
pub(crate) mod util;
mod world_interaction;

pub(crate) use system_set::GameSystemSet;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]

enum GameState {
    /// During loading state loading_plugin loads assets
    #[default]
    Loading,
    /// During playing state, logic is executed
    Playing,
    /// Menu created and awaiting user interaction
    Menu,
}

/// Main entrypoint for dark_forest
///
/// The top-level plugins are:
/// - [`system_set::plugin`]: Sets up the system set used to order systems across DarkForest.
/// - [`bevy_config::plugin`]: Sets up the bevy configuration.
/// - [`menu::plugin`]: Handles the menu.
/// - [`movement::plugin`]: Handles the movement of entities.
/// - [`player_control::plugin`]: Handles the player's control.
/// - [`world_interaction::plugin`]: Handles the interaction of entities with the world.
/// - [`level_instantiation::plugin`]: Handles the creation of levels and objects.
/// - [`file_system_interaction::plugin`]: Handles the loading and saving of games.
/// - [`shader::plugin`]: Handles the shaders.
/// - [`dev::plugin`]: Handles the dev tools.
/// - [`ingame_menu::plugin`]: Handles the ingame menu accessed via ESC.
/// - [`particles::plugin`]: Handles the particle system.

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            system_set::plugin,
            bevy_config::plugin,
            menu::plugin,
            movement::plugin,
            player_control::plugin,
            world_interaction::plugin,
            level_instantiation::plugin,
            file_system_interaction::plugin,
            shader::plugin,
            ingame_menu::plugin,
            particles::plugin,
            #[cfg(feature = "dev")]
            dev::plugin,
        ));
    }
}
