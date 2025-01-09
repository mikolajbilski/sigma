use bevy::prelude::Component;

#[derive(Component)]
pub(crate) enum ButtonTypeMarker {
    MainMenu,
    StartGame,
    DisplayStats,
    Exit,
}
