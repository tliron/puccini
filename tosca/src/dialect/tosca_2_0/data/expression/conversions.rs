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
