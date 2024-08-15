use bevy::prelude::*;

mod blender_workflow;
mod map;
pub(crate) mod on_spawn;

/// Handles creation of levels and objects. Split into the following sub-plugings:
///  - [`map::pluign`] handles loading of level files and orchestrates the spawning of the objects therein.
/// - [``]
