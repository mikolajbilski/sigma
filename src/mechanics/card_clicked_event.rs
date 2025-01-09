use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub(crate) struct CardClickedEvent {
    pub(crate) entity: Entity,
}
