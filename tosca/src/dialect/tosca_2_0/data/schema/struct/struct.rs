use super::{
    super::{
        super::{data_kind::*, expression::*},
        reference::*,
    },
    field::*,
};

use {compris::annotate::*, depiction::*, kutil::std::immutable::*, std::collections::*};

//
// StructSchema
//

/// Struct schema.
#[derive(Clone, Debug, Default, Depict, Eq)]
pub struct StructSchema<AnnotatedT> {
    /// Fields.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub fields: BTreeMap<ByteString, StructSchemaField>,

    /// Default.
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// Validation.
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,
}

impl<AnnotatedT> StructSchema<AnnotatedT> {
    /// Update reference.
    pub fn update_reference(&mut self, old: SchemaReference, new: SchemaReference) {
        for field in self.fields.values_mut() {
            field.update_reference(old, new);
        }
    }

    /// Into expression.
    pub fn into_expression(self, positions: &SchemaReferencePositions) -> Expression<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), DataKind::Struct.as_str().into());

        let fields: BTreeMap<_, _> = self
            .fields
            .into_iter()
            .map(|(field_name, field)| (field_name.into(), field.into_expression(positions)))
            .collect();

        map.insert("fields".into(), fields.into());

        if let Some(default) = self.default {
            map.insert("default".into(), default);
        }

        if let Some(validation) = self.validation {
            map.insert("validation".into(), validation.lazy_assert());
        }

        map.into()
    }
}

impl<AnnotatedT> PartialEq for StructSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.fields == other.fields) && (self.default == other.default) && (self.validation == other.validation)
    }
}
