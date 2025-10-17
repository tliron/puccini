use super::super::{catalog::*, compile::*, errors::*};

use {compris::annotate::*, kutil::std::error::*};

impl Catalog {
    /// Compile service template to Floria.
    pub fn compile_service_template(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>> {
        let source = unwrap_or_give_and_return!(self.get_source(context.source_id), context.errors, Ok(None));
        let dialect = unwrap_or_give_and_return!(self.get_dialect_ref(&source.dialect_id), context.errors, Ok(None));
        dialect.compile_source(context)
    }
}
