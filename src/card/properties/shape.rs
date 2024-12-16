#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Shape {
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
