use super::super::dispatch::*;

use compris::{annotate::*, normal::*};

impl<AnnotatedT> super::Expression<AnnotatedT> {
    /// To Floria property value and updater.
    pub fn to_floria_property_fields(&self) -> (Option<Variant<WithoutAnnotations>>, Option<floria::Expression>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let floria_expression: floria::Expression = self.into();
        match floria_expression {
            floria::Expression::Literal(literal) => (Some(literal), None),
            floria::Expression::Call(_) => (None, Some(floria_expression)),
        }
    }

    /// To Floria property validator.
    pub fn to_floria_property_validator(&self) -> Option<floria::Expression>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        match self {
            Self::Literal(variant @ Variant::Boolean(_)) => Some(variant.clone().into()),
            Self::Call(call) => Some(call.into()),
            _ => None,
        }
    }
}

impl<AnnotatedT> Into<floria::Expression> for &super::Expression<AnnotatedT>
where
    AnnotatedT: Annotated + Clone,
{
    fn into(self) -> floria::Expression {
        if let Some(literal) = self.to_literal_variant() {
            return floria::Expression::Literal(literal);
        }

        match self {
            super::Expression::Call(call) => call.into(),

            super::Expression::List(list) => {
                let arguments = list.into_iter().map(|item| item.into()).collect();
                floria::Call::new(get_dispatch_name("_list"), arguments).into()
            }

            super::Expression::Map(map) => {
                let arguments = map.into_iter().map(|(key, value)| vec![key.into(), value.into()]).flatten().collect();
                floria::Call::new(get_dispatch_name("_map"), arguments).into()
            }

            _ => panic!("this case should have been handled elsewhere"),
        }
    }
}
