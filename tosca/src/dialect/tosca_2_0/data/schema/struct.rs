use super::{
    super::{data_kind::*, expression::*},
    value::*,
};

use {
    kutil::{cli::depict::*, std::immutable::*},
    std::collections::*,
};

//
// StructSchema
//

/// Struct schema.
#[derive(Clone, Debug, Default, Depict)]
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

impl<AnnotatedT> PartialEq for StructSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.fields == other.fields) && (self.default == other.default) && (self.validation == other.validation)
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for StructSchema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), DataKind::Struct.as_str().into());

        let fields: BTreeMap<_, _> = self
            .fields
            .into_iter()
            .map(|(field_name, field)| {
                (
                    field_name.into(),
                    if field.required {
                        // Just the schema
                        // ("required" defaults to true, so we don't need to specify it)
                        (field.reference as u64).into()
                    } else {
                        // List = [schema, required]
                        vec![(field.reference as u64).into(), false.into()].into()
                    },
                )
            })
            .collect();

        map.insert("fields".into(), fields.into());

        if let Some(default) = self.default {
            map.insert("default".into(), default);
        }

        if let Some(validation) = self.validation {
            map.insert("validation".into(), validation.into_lazy());
        }

        map.into()
    }
}

//
// StructSchemaField
//

/// Struct schema field.
#[derive(Clone, Debug, Depict, PartialEq)]
pub struct StructSchemaField {
    /// Reference.
    pub reference: SchemaReference,

    /// Required.
    pub required: bool,
}

impl StructSchemaField {
    /// Constructor.
    pub fn new(reference: SchemaReference, required: bool) -> Self {
        Self { reference, required }
    }
}

impl From<SchemaReference> for StructSchemaField {
    fn from(reference: SchemaReference) -> Self {
        Self { reference, required: true }
    }
}
