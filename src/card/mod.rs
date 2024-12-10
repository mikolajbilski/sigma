pub mod deck;
pub use deck::Deck;
pub mod properties;
use properties::*;

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Card {
    shape: Shape,
    color: CardColor,
    count: Count,
    fill: Fill,
}

impl Card {
    //TODO: temp (testing)
    pub fn default() -> Self {
        Card {
            shape: Shape::Oval,
            color: CardColor::Red,
            count: Count::One,
            fill: Fill::Opaque,
        }
    }

    fn new(shape: Shape, color: CardColor, count: Count, fill: Fill) -> Self {
        Card {
            shape,
            color,
            count,
            fill,
        }
    }

    pub fn get_texture_color(&self) -> Color {
        self.color.as_color()
    }

    pub fn get_count(&self) -> i32 {
        self.count.as_number()
    }

    pub fn get_asset_path(&self) -> String {
        format!(
            "sprites/cards/{}_{}.png",
            self.shape.as_name(),
            self.fill.as_name()
        )
    }

    pub fn generate_card_entity(
        &self,
        asset_server: &Res<AssetServer>,
    ) -> impl FnOnce(&mut ChildBuilder) {
        let background_texture = asset_server.load("sprites/general/card_background.png");
        let content_texture = asset_server.load(self.get_asset_path());
        let count = self.get_count();
        let color = self.get_texture_color();

        move |parent: &mut ChildBuilder| {
            // Spawn the background
            parent
                .spawn(SpriteBundle {
                    texture: background_texture.clone(),
                    ..Default::default()
                })
                .with_children(|background| {
                    // Spawn each content sprite as a child

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
                });
        }
    }

    pub(crate) fn generate_all() -> Vec<Self> {
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
