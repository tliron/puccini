use super::{
    super::{super::super::grammar::*, data::*, dialect::*, entities::*},
    plugin::*,
};

use {compris::annotate::*, kutil::std::error::*, std::mem::*};

impl<AnnotatedT> Call<AnnotatedT> {
    /// Compile to a Floria expression.
    pub fn compile(
        self,
        context: &mut CompilationContext<'_>,
    ) -> Result<floria::Expression, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let (function, source) = must_unwrap_give!(
            context.catalog.entity::<FunctionDefinition<AnnotatedT>, _>(FUNCTION, &self.function, context.source_id),
            context.errors.with_fallback_annotations(self.annotations())
        );

        // TODO: find signature

        let mut plugin = {
            if let Some(plugin) = function.floria_plugin(&mut context.with_source(&source.source_id))? {
                plugin
            } else {
                return Ok(floria::Expression::Undefined);
            }
        };

        // TODO: other artifact types?

        let function = take(&mut plugin.function).unwrap_or(self.function.name.0);

        let Some(plugin_id) = plugin.get_or_create(None, context)? else {
            return Ok(floria::Expression::Undefined);
        };

        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in self.arguments {
            let argument = unwrap_or_give!(argument.compile(context), context.errors, floria::Expression::Undefined);
            arguments.push(argument);
        }

        Ok(floria::Call::new(plugin_id, function, arguments, self.kind)?.into())
    }
}
