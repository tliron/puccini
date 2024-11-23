use std::{fmt, sync::atomic::*};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

//
// SchemaReference
//

/// Schema reference.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchemaReference(pub usize);

impl Default for SchemaReference {
    fn default() -> Self {
        SchemaReference(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl fmt::Display for SchemaReference {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "ID:{}", self.0)
    }
}
