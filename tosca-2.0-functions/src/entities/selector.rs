use std::fmt;

//
// ToscaSelector
//

/// TOSCA selector.
#[derive(Clone, Debug)]
pub enum ToscaSelector {
    /// Index.
    Index(usize),

    /// All.
    All,
}

impl Default for ToscaSelector {
    fn default() -> Self {
        Self::Index(0)
    }
}

impl fmt::Display for ToscaSelector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Index(index) => fmt::Display::fmt(index, formatter),
            Self::All => fmt::Display::fmt("ALL", formatter),
        }
    }
}
