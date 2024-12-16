use bevy::color::Color;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CardColor {
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
            CardColor::Red => Color::srgb_u8(0xff, 0x01, 0x01),
            CardColor::Green => Color::srgb_u8(0x00, 0x80, 0x02),
            CardColor::Purple => Color::srgb_u8(0x80, 0x00, 0x80),
        }
    }
}

impl Display for CardColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            CardColor::Red => "red",
            CardColor::Green => "green",
            CardColor::Purple => "purple",
        };
        write!(f, "{}", s)
    }
}
