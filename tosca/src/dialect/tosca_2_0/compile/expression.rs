use super::super::{super::super::grammar::*, data::*};

use {compris::annotate::*, problemo::*, std::collections::*};

impl<AnnotatedT> Expression<AnnotatedT> {
    /// Compile to a Floria expression.
    pub fn compile(self, context: &mut CompilationContext) -> Result<floria::Expression, Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        Ok(match self {
            Expression::Simple(simple) => simple.into(),

            Expression::List(list) => {
                let mut floria_list = Vec::with_capacity(list.len());
                for item in list {
                    if let Some(item) = item.compile(context).give_ok(&mut context.problems)? {
                        floria_list.push(item);
                    }
                }
                floria_list.into()
            }

            Expression::Map(map) => {
                let mut floria_map = BTreeMap::default();
                for (key, value) in map {
                    if let Some(key) = key.compile(context).give_ok(&mut context.problems)?
                        && let Some(value) = value.compile(context).give_ok(&mut context.problems)?
                    {
                        floria_map.insert(key, value);
                    }
                }
                floria_map.into()
            }

            Expression::Call(call) => {
                give_unwrap!(call.compile(context), &mut context.problems, floria::Expression::Undefined)
            }
        })
    }
}
