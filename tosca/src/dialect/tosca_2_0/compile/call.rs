use super::{
    super::{super::super::grammar::*, data::*, dialect::*, entities::*},
    plugin::*,
};

use {compris::annotate::*, problemo::*, std::mem::*};

impl<AnnotatedT> Call<AnnotatedT> {
    /// Compile to a Floria expression.
    pub fn compile(self, context: &mut CompilationContext) -> Result<floria::Expression, Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let (function, source) = give_unwrap!(
            context.catalog.entity::<FunctionDefinition<AnnotatedT>>(FUNCTION, &self.function, context.source_id),
            &mut context.problems.with_fallback_annotations(self.annotations())
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
            let argument =
                give_unwrap!(argument.compile(context), &mut context.problems, floria::Expression::Undefined);
            arguments.push(argument);
        }

        Ok(floria::Call::new(plugin_id, function, arguments, self.kind)?.into())
    }
}
