use bevy::prelude::*;
use dark_forest::GamePlugin;

#[derive(Component)]
struct MyCameraMarker;

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 12.0, 16.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCameraMarker,
    ));
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GamePlugin)
    .add_systems(Startup, camera_setup)
    .run();
}
