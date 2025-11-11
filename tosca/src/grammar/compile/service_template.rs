use super::super::{catalog::*, compile::*, errors::*};

use {compris::annotate::*, kutil::std::error::*};

impl Catalog {
    /// Compile service template to Floria.
    pub fn compile_service_template(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>> {
        let source = must_unwrap_give!(self.source(context.source_id), context.errors);
        let dialect = must_unwrap_give!(self.get_dialect_ref(&source.dialect_id), context.errors);
        dialect.compile_source(context)
    }
}
