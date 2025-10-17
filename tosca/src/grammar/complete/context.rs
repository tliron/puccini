use super::super::{catalog::*, errors::*, source::*};

//
// CompletionContext
//

/// Completion context.
pub struct CompletionContext<'own> {
    /// Catalog.
    pub catalog: &'own mut Catalog,

    /// Source ID.
    pub source_id: &'own SourceID,

    /// Errors.
    pub errors: ToscaErrorReceiverRef<'own>,
}

impl<'own> CompletionContext<'own> {
    /// Constructor.
    pub fn new(catalog: &'own mut Catalog, source_id: &'own SourceID, errors: ToscaErrorReceiverRef<'own>) -> Self {
        Self { catalog, source_id, errors }
    }
}

/// [CompletionContext] with errors.
#[macro_export]
macro_rules! context_with_errors {
    ($context:expr, $errors:expr) => {{
        use ::kutil::std::error::*;
        &mut $crate::grammar::CompletionContext::new($context.catalog, $context.source_id, $errors.to_ref())
    }};
}

#[allow(unused_imports)]
pub use context_with_errors;
