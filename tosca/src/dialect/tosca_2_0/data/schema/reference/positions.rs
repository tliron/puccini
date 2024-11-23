use super::{
    super::{super::expression::*, schema::*},
    reference::*,
};

use kutil::std::collections::*;

//
// SchemaReferencePositions
//

/// Schema reference positions.
#[derive(Debug)]
pub struct SchemaReferencePositions(pub FastHashMap<SchemaReference, usize>);

impl SchemaReferencePositions {
    /// Constructor.
    pub fn new<AnnotatedT>(schema: &Schema<AnnotatedT>) -> Self {
        let mut positions = FastHashMap::default();

        // This works because value_schemas is ordered (a BTreeMap)
        for (position, reference) in schema.value_schemas.keys().enumerate() {
            positions.insert(*reference, position);
        }

        Self(positions)
    }

    /// Position for a reference.
    pub fn position(&self, reference: SchemaReference) -> usize {
        self.0.get(&reference).map(|position| *position).expect("dangling schema reference")
    }

    /// Position, as an expression, for a reference.
    pub fn expression<AnnotatedT>(&self, reference: SchemaReference) -> Expression<AnnotatedT>
    where
        AnnotatedT: Default,
    {
        (self.position(reference) as u64).into()
    }
}
