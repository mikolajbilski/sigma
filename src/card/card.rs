use crate::{card_clicked_event::CardClickedEvent, selection_manager::CheckSelectedEvent};

use super::{highlight_marker::HighlightMarker, properties::*};
use bevy::{math::vec3, prelude::*};
use color::CardColor;
use count::Count;
use fill::Fill;
use shape::Shape;
use std::fmt::Display;

#[derive(Component, Debug, Clone, Copy)]
pub(crate) struct Card {
    pub(crate) shape: Shape,
    pub(crate) color: CardColor,
    pub(crate) count: Count,
    pub(crate) fill: Fill,

    displayed: bool,
    selected: bool,
    to_remove: bool,
}

pub(crate) fn flip_selection(
    mut card_entity_query: Query<(Entity, &mut Card)>,
    mut card_event: EventReader<CardClickedEvent>,
    mut highlight_query: Query<(Entity, &Parent, &mut Visibility), With<HighlightMarker>>,
    parent_query: Query<&Parent>,
    mut ev_check_selected: EventWriter<CheckSelectedEvent>,
) {
    for event in card_event.read() {
        let entity = event.entity;
        let mut card = card_entity_query.get_mut(entity).unwrap().1;
        card.flip_selection();
        ev_check_selected.send(CheckSelectedEvent {});

        for (_, parent, mut visibility) in highlight_query.iter_mut() {
            if let Ok(grandparent) = parent_query.get(parent.get()) {
                if grandparent.get() == entity {
                    if card.is_selected() {
                        *visibility = Visibility::Inherited;
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                    break;
                }
            }
        }
    }
}

impl Card {
    const SPRITE_WIDTH: f32 = 180.0;
    const SPRITE_HEIGHT: f32 = 108.0;

    fn new(shape: Shape, color: CardColor, count: Count, fill: Fill) -> Self {
        Card {
            shape,
            color,
            count,
            fill,

            displayed: false,
            selected: false,
            to_remove: false,
        }
    }

    pub(crate) fn get_texture_color(&self) -> Color {
        self.color.as_color()
    }

    pub(crate) fn get_count(&self) -> i32 {
        self.count.as_number()
    }

    pub(crate) fn mark_for_removal(&mut self) {
        self.to_remove = true;
    }

    pub(crate) fn should_remove(&self) -> bool {
        self.to_remove
    }

    pub(crate) fn get_asset_path(&self) -> String {
        format!(
            "sprites/cards/{}_{}.png",
            self.shape.as_name(),
            self.fill.as_name()
        )
    }

    pub(crate) fn flip_selection(&mut self) {
        self.selected = !self.selected;
    }

    pub(crate) fn is_selected(&self) -> bool {
        self.selected
    }

    pub(crate) fn set_displayed(&mut self, val: bool) {
        self.displayed = val;
    }

    pub(crate) fn is_displayed(&self) -> bool {
        self.displayed
    }

    pub(crate) fn generate_card_entity(
        &self,
        asset_server: &Res<AssetServer>,
    ) -> impl FnOnce(&mut ChildBuilder) {
        let background_texture = asset_server.load("sprites/general/card_background.png");
        let content_texture = asset_server.load(self.get_asset_path());
        let count = self.get_count();
        let color = self.get_texture_color();
        let highlight_texture = asset_server.load("sprites/general/card_selection.png");

        move |parent: &mut ChildBuilder| {
            parent
                .spawn((SpriteBundle {
                    texture: background_texture.clone(),
                    transform: Transform::from_scale(vec3(
                        Card::SPRITE_WIDTH / 300.0,
                        Card::SPRITE_HEIGHT / 180.0,
                        1.0,
                    )),
                    ..Default::default()
                },))
                .with_children(|background| {
                    let positions = match count {
                        1 => vec![0],
                        2 => vec![-45, 45],
                        3 => vec![-90, 0, 90],
                        _ => vec![0],
                    };
                    for i in positions.iter() {
                        background.spawn(SpriteBundle {
                            texture: content_texture.clone(),
                            transform: Transform::from_xyz(*i as f32, 0.0, 1.0),
                            sprite: Sprite { color, ..default() },
                            ..Default::default()
                        });
                    }
                    background.spawn((
                        SpriteBundle {
                            texture: highlight_texture.clone(),
                            visibility: Visibility::Hidden,
                            ..Default::default()
                        },
                        HighlightMarker {},
                    ));
                });
        }
    }

    // Return all possible cards
    pub(crate) fn all_cards() -> Vec<Self> {
        let shapes = Shape::all_variants();
        let colors = CardColor::all_variants();
        let counts = Count::all_variants();
        let fills = Fill::all_variants();

        let mut all = vec![];

        for shape in shapes.iter() {
            for color in colors.iter() {
                for count in counts.iter() {
                    for fill in fills.iter() {
                        all.push(Card::new(*shape, *color, *count, *fill));
                    }
                }
            }
        }

        all
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
            && self.color == other.color
            && self.count == other.count
            && self.fill == other.fill
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.count.as_number(),
            self.fill.as_name(),
            self.color,
            self.shape.as_name()
        )
    }
}
