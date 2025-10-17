use super::super::{super::super::grammar::*, data::*};

use {compris::annotate::*, kutil::std::error::*, std::collections::*};

impl<AnnotatedT> Expression<AnnotatedT> {
    /// Compile to a Floria expression.
    pub fn compile(
        self,
        context: &mut CompilationContext<'_>,
    ) -> Result<floria::Expression, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        Ok(match self {
            Expression::Simple(simple) => simple.into(),

            Expression::List(list) => {
                let mut floria_list = Vec::with_capacity(list.len());
                for item in list {
                    match item.compile(context) {
                        Ok(item) => floria_list.push(item),
                        Err(error) => {
                            context.errors.give(error)?;
                        }
                    }
                }
                floria_list.into()
            }

            Expression::Map(map) => {
                let mut floria_map = BTreeMap::default();
                for (key, value) in map {
                    match key.compile(context) {
                        Ok(key) => match value.compile(context) {
                            Ok(value) => {
                                floria_map.insert(key, value);
                            }
                            Err(error) => {
                                context.errors.give(error)?;
                            }
                        },
                        Err(error) => {
                            context.errors.give(error)?;
                        }
                    }
                }
                floria_map.into()
            }

            Expression::Call(call) => match call.compile(context) {
                Ok(call) => call,
                Err(error) => {
                    context.errors.give(error)?;
                    floria::Expression::Undefined
                }
            },
        })
    }
}
