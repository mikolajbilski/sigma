use bevy::prelude::*;

#[derive(Component)]
pub struct Card {
    shape: Shape,
    color: Color,
    count: Count,
    fill: Fill,
}

impl Card {
    pub fn new() -> Self {
        Card {
            shape: Shape::Oval,
            color: Color::Red,
            count: Count::One,
            fill: Fill::Opaque,
        }
    }
}

enum Shape {
    Oval,
    Diamond,
    Squiggle,
}

enum Color {
    Red,
    Green,
    Purple,
}

enum Count {
    One,
    Two,
    Three,
}

enum Fill {
    Opaque,
    Striped,
    Transparent,
}