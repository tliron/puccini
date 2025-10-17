use super::super::{super::super::grammar::*, data::*, dialect::*, entities::*};

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

        let (_file, prefix) = match function.plugin() {
            Ok(plugin) => match plugin {
                Some((file, prefix)) => (file, prefix),
                None => {
                    // TODO: support for other artifacts?
                    return Ok(floria::Expression::Undefined);
                }
            },

            Err(error) => {
                context.errors.give(error.with_annotations_from(&self).into_annotated())?;
                return Ok(floria::Expression::Undefined);
            }
        };

        let Some(prefix) = prefix else {
            context
                .errors
                .give(MissingRequiredError::new("floria-prefix".into(), None).with_annotations_from(&self))?;
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

        Ok(floria::Call::new(prefix, self.function.name.0, arguments, self.kind).into())
    }
}
