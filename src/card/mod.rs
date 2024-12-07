pub mod deck;
pub use deck::Deck;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Card {
    shape: Shape,
    color: Color,
    count: Count,
    fill: Fill,
}

impl Card {
    pub fn default() -> Self {
        Card {
            shape: Shape::Oval,
            color: Color::Red,
            count: Count::One,
            fill: Fill::Opaque,
        }
    }

    fn new(shape: Shape, color: Color, count: Count, fill: Fill) -> Self {
        Card {
            shape,
            color,
            count,
            fill,
        }
    }

    pub(crate) fn generate_all() -> Vec<Self> {
        let shapes = Shape::all_variants();
        let colors = Color::all_variants();
        let counts = Count::all_variants();
        let fills   = Fill::all_variants();

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

#[derive(Clone, Copy, Debug)]
enum Shape {
    Oval,
    Diamond,
    Squiggle,
}

impl Shape {
    pub(crate) fn all_variants() -> Vec<Shape> {
        vec![Shape::Oval, Shape::Diamond, Shape::Squiggle]
    }
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Red,
    Green,
    Purple,
}

impl Color {
    pub(crate) fn all_variants() -> Vec<Color> {
        vec![Color::Red, Color::Green, Color::Purple]
    }
}

#[derive(Clone, Copy, Debug)]
enum Count {
    One,
    Two,
    Three,
}

impl Count {
    pub(crate) fn all_variants() -> Vec<Count> {
        vec![Count::One, Count::Two, Count::Three]
    }
}

#[derive(Clone, Copy, Debug)]
enum Fill {
    Opaque,
    Striped,
    Transparent,
}

impl Fill {
    pub(crate) fn all_variants() -> Vec<Fill> {
        vec![Fill::Opaque, Fill::Striped, Fill::Transparent]
    }
}