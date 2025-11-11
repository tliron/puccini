use super::{
    super::{super::super::grammar::*, data::*, dialect::*, entities::*},
    plugin::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl<AnnotatedT> Call<AnnotatedT> {
    /// Compile to a Floria expression.
    pub fn compile(
        self,
        context: &mut CompilationContext<'_>,
    ) -> Result<floria::Expression, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let function = match context.catalog.entity::<FunctionDefinition<AnnotatedT>, _>(
            FUNCTION,
            &self.function,
            context.source_id,
        ) {
            Ok(function) => function,
            Err(error) => {
                context.errors.give(error.with_annotations_from(&self))?;
                return Ok(floria::Expression::Undefined);
            }
        };

        // TODO: find signature

        let (plugin_url, global) = match function.plugin_url() {
            Ok(Some(file)) => file,

            Ok(None) => {
                // TODO: support for other artifacts?
                return Ok(floria::Expression::Undefined);
            }

            Err(error) => {
                context.errors.give(error.with_annotations_from(&self).into_annotated())?;
                return Ok(floria::Expression::Undefined);
            }
        };

        let directory = if global { Default::default() } else { context.directory.clone() };
        let Some(plugin_id) = get_or_create_plugin_by_url(plugin_url, directory, None, context)? else {
            return Ok(floria::Expression::Undefined);
        };

        let mut arguments = Vec::with_capacity(self.arguments.len());
        for argument in self.arguments {
            let argument = match argument.compile(context) {
                Ok(argument) => argument,
                Err(error) => {
                    context.errors.give(error)?;
                    floria::Expression::Undefined
                }
            };
            arguments.push(argument);
        }

        Ok(floria::Call::new(plugin_id, self.function.name.0, arguments, self.kind)?.into())
    }
}
