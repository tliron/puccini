use super::super::{catalog::*, compile::*};

use problemo::*;

impl Catalog {
    /// Compile service template to Floria.
    pub fn compile_service_template_with_annotations(
        &self,
        context: &mut CompilationContext,
    ) -> Result<Option<floria::ID>, Problem> {
        let source = give_unwrap!(self.source(context.source_id), &mut context.problems);
        let dialect = give_unwrap!(self.get_dialect_ref(&source.dialect_id), &mut context.problems);
        dialect.compile_source_with_annotations(context)
    }

    /// Compile service template to Floria without annotations.
    pub fn compile_service_template_without_annotations(
        &self,
        context: &mut CompilationContext,
    ) -> Result<Option<floria::ID>, Problem> {
        let source = give_unwrap!(self.source(context.source_id), &mut context.problems);
        let dialect = give_unwrap!(self.get_dialect_ref(&source.dialect_id), &mut context.problems);
        dialect.compile_source_without_annotations(context)
    }
}
