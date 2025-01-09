use bevy::prelude::States;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum AppState {
    #[default]
    Menu,
    Game,
}
