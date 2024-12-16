#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Fill {
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
