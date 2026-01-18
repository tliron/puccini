use super::super::{catalog::*, source::*};

use problemo::*;

//
// CompletionContext
//

/// Completion context.
pub struct CompletionContext<'context> {
    /// Catalog.
    pub catalog: &'context mut Catalog,

    /// Source ID.
    pub source_id: &'context SourceID,

    /// Problems.
    pub problems: ProblemReceiverRef<'context>,
}

impl<'context> CompletionContext<'context> {
    /// Constructor.
    pub fn new(
        catalog: &'context mut Catalog,
        source_id: &'context SourceID,
        problems: ProblemReceiverRef<'context>,
    ) -> Self {
        Self { catalog, source_id, problems }
    }
}

/// [CompletionContext] with problems.
#[macro_export]
macro_rules! context_with_problems {
    ($context:expr, $problems:expr) => {{
        use problemo::*;
        &mut $crate::grammar::CompletionContext::new($context.catalog, $context.source_id, $problems.as_ref())
    }};
}

#[allow(unused_imports)]
pub use context_with_problems;
