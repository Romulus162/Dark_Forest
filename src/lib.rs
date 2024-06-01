// #![allow(clippy::too_many_arguments, clippy::type_complexity)]
// #![allow(rustdoc::private_intra_doc_links)]


//! Dark Forest is split into many plugins with their own set of responsibilities.
//! This is an organizational measure I have addapted and basing off of the fox_trot framework
//! everyting seen here, is a reverse engineer project of that framework.

use bevy::prelude::*;
mod menu;
mod ingame_menu;
mod player_control;

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
/// - [`menu::plugin`]: Handles the menu.
/// - [`ingame_menu::plugin`]: Handles the ingame menu acces via ESC.

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            menu::plugin,
            ingame_menu::plugin,
        ));
    }
}
