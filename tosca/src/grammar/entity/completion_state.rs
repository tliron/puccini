//
// CompletionState
//

/// Completion state.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompletionState {
    /// Incomplete.
    #[default]
    Incomplete,

    /// Complete.
    Complete,

    /// Cannot complete.
    Cannot,
}
