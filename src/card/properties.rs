use bevy::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Oval,
    Diamond,
    Squiggle,
}

impl Shape {
    pub(crate) fn all_variants() -> Vec<Shape> {
        vec![Shape::Oval, Shape::Diamond, Shape::Squiggle]
    }

    pub(crate) fn as_name(&self) -> &str {
        match self {
            Shape::Oval => "oval",
            Shape::Diamond => "diamond",
            Shape::Squiggle => "squiggle",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardColor {
    Red,
    Green,
    Purple,
}

impl CardColor {
    pub(crate) fn all_variants() -> Vec<CardColor> {
        vec![CardColor::Red, CardColor::Green, CardColor::Purple]
    }

    pub(crate) fn as_color(&self) -> Color {
        match self {
            CardColor::Red => Color::srgb(1.0, 0.0, 0.0),
            CardColor::Green => Color::srgb(0.0, 1.0, 0.0),
            CardColor::Purple => Color::srgb(1.0, 0.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Count {
    One,
    Two,
    Three,
}

impl Count {
    pub(crate) fn all_variants() -> Vec<Count> {
        vec![Count::One, Count::Two, Count::Three]
    }

    pub(crate) fn as_number(&self) -> i32 {
        match self {
            Count::One => 1,
            Count::Two => 2,
            Count::Three => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fill {
    Opaque,
    Striped,
    Transparent,
}

impl Fill {
    pub(crate) fn all_variants() -> Vec<Fill> {
        vec![Fill::Opaque, Fill::Striped, Fill::Transparent]
    }

    pub(crate) fn as_name(&self) -> &str {
        match self {
            Fill::Opaque => "opaque",
            Fill::Striped => "striped",
            Fill::Transparent => "transparent",
        }
    }
}
