use super::dialect::*;

use {
    kutil::std::any::*,
    std::{any::*, fmt},
};

//
// DialectRef
//

/// Common reference type for [Dialect].
pub type DialectRef = Box<dyn Dialect>;

impl fmt::Debug for DialectRef {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "DialectRef")
    }
}

impl<GrammarT> From<GrammarT> for DialectRef
where
    GrammarT: 'static + Dialect,
{
    fn from(value: GrammarT) -> Self {
        Box::new(value)
    }
}

impl AsAnyRef for DialectRef {
    fn as_any_ref(&self) -> Option<&dyn Any> {
        Some(self.as_ref())
    }
}
