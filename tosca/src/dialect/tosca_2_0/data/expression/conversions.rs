use super::{call::*, expression::*};

use {compris::normal::*, duplicate::*, kutil::std::immutable::*, std::collections::*};

#[duplicate_item(
  FromT;
  [i64];
  [u64];
  [f64];
  [bool];
  [ByteString];
  [String];
  [Bytes];
)]
impl<AnnotatedT> From<FromT> for Expression<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(value: FromT) -> Self {
        Self::Simple(value.into())
    }
}

impl<AnnotatedT> From<&'static str> for Expression<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn from(string: &'static str) -> Self {
        ByteString::from_static(string).into()
    }
}

impl<AnnotatedT> From<Variant<AnnotatedT>> for Expression<AnnotatedT> {
    fn from(variant: Variant<AnnotatedT>) -> Self {
        Self::Simple(variant)
    }
}

impl<AnnotatedT> From<Vec<Expression<AnnotatedT>>> for Expression<AnnotatedT> {
    fn from(list: Vec<Expression<AnnotatedT>>) -> Self {
        Self::List(list)
    }
}

impl<AnnotatedT> From<BTreeMap<Expression<AnnotatedT>, Expression<AnnotatedT>>> for Expression<AnnotatedT> {
    fn from(map: BTreeMap<Expression<AnnotatedT>, Expression<AnnotatedT>>) -> Self {
        Self::Map(map)
    }
}

impl<AnnotatedT> From<Call<AnnotatedT>> for Expression<AnnotatedT> {
    fn from(call: Call<AnnotatedT>) -> Self {
        Self::Call(call)
    }
}

impl<AnnotatedT> Into<floria::Expression> for Expression<AnnotatedT> {
    fn into(self) -> floria::Expression {
        match self {
            Expression::Simple(simple) => simple.into(),

            Expression::List(list) => {
                let list: Vec<floria::Expression> = list.into_iter().map(|item| item.into()).collect();
                list.into()
            }

            Expression::Map(map) => {
                let map: BTreeMap<floria::Expression, floria::Expression> =
                    map.into_iter().map(|(key, value)| (key.into(), value.into())).collect();
                map.into()
            }

            Expression::Call(call) => call.into(),
        }
    }
}
