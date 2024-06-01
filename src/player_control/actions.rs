use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Actionlike, Reflect, Default)]
pub(crate) enum UiAction {
    #[default]
    TogglePause,
}
