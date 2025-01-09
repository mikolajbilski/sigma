use bevy::prelude::Component;

#[derive(Component)]
pub(crate) enum ButtonTypeMarker {
    StartGame,
    DisplayStats,
    Exit,
}