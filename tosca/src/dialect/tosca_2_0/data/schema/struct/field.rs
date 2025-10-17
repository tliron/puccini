use super::super::{super::expression::*, reference::*};

use depiction::*;

//
// StructSchemaField
//

/// Struct schema field.
#[derive(Clone, Debug, Depict, Eq, PartialEq)]
pub struct StructSchemaField {
    /// Reference.
    #[depict(style(meta))]
    pub reference: SchemaReference,

    /// Required.
    #[depict(style(symbol))]
    pub required: bool,
}

impl StructSchemaField {
    /// Constructor.
    pub fn new(reference: SchemaReference, required: bool) -> Self {
        Self { reference, required }
    }

    /// Update reference.
    pub fn update_reference(&mut self, old: SchemaReference, new: SchemaReference) {
        if self.reference == old {
            self.reference = new;
        }
    }

    /// Into expression.
    pub fn into_expression<AnnotatedT>(self, index: &SchemaReferencePositions) -> Expression<AnnotatedT>
    where
        AnnotatedT: Default,
    {
        if self.required {
            // Just the schema
            // ("required" defaults to true, so we don't need to specify it)
            index.expression(self.reference)
        } else {
            // List = [schema, required]
            vec![index.expression(self.reference), false.into()].into()
        }
    }
}

impl From<SchemaReference> for StructSchemaField {
    fn from(reference: SchemaReference) -> Self {
        Self { reference, required: true }
    }
}
