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
                    if let Some(item) = ok_give!(item.compile(context), context.errors) {
                        floria_list.push(item);
                    }
                }
                floria_list.into()
            }

            Expression::Map(map) => {
                let mut floria_map = BTreeMap::default();
                for (key, value) in map {
                    if let Some(key) = ok_give!(key.compile(context), context.errors)
                        && let Some(value) = ok_give!(value.compile(context), context.errors)
                    {
                        floria_map.insert(key, value);
                    }
                }
                floria_map.into()
            }

            Expression::Call(call) => {
                unwrap_or_give!(call.compile(context), context.errors, floria::Expression::Undefined)
            }
        })
    }
}
