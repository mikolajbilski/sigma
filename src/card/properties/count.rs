#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Count {
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
