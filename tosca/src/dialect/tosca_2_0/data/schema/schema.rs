use super::{
    super::{super::super::super::grammar::*, data_kind::*, expression::*},
    list::*,
    map::*,
    primitive::*,
    reference::*,
    scalar::*,
    r#struct::*,
    value::*,
};

use {
    compris::annotate::*,
    kutil::{cli::depict::*, std::collections::*},
    std::collections::*,
};

//
// Schema
//

/// Schema.
#[derive(Clone, Debug, Default, Depict)]
pub struct Schema<AnnotatedT> {
    /// Value schemas.
    ///
    /// Ordered by the age of the schema references, oldest to newest. This ensures that the root
    /// is first.
    #[depict(iter(kv), key_style(number), as(depict))]
    pub value_schemas: BTreeMap<SchemaReference, ValueSchema<AnnotatedT>>,

    /// Data type references.
    #[depict(iter(kv), key_style(name), style(number))]
    pub data_type_references: FastHashMap<FullName, SchemaReference>,
}

impl<AnnotatedT> Schema<AnnotatedT> {
    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.value_schemas.is_empty()
    }

    /// Add a value schema and return its reference.
    pub fn add(&mut self, schema: ValueSchema<AnnotatedT>) -> SchemaReference {
        let reference = Default::default();
        self.value_schemas.insert(reference, schema);
        reference
    }

    /// If the schema already exists then return its reference. Otherwise add it at the provided
    /// reference and return it.
    pub fn add_unique(&mut self, reference: SchemaReference, schema: ValueSchema<AnnotatedT>) -> SchemaReference {
        for (reference, existing) in &self.value_schemas {
            if schema == *existing {
                return *reference;
            }
        }

        self.value_schemas.insert(reference, schema);
        reference
    }

    /// Update reference.
    pub fn update_reference(&mut self, old: SchemaReference, new: SchemaReference) {
        for value_schema in self.value_schemas.values_mut() {
            value_schema.update_reference(old, new);
        }
    }

    /// Dereference.
    pub fn dereference(&self, reference: SchemaReference) -> Option<&ValueSchema<AnnotatedT>> {
        match self.value_schemas.get(&reference) {
            Some(ValueSchema::Reference(reference)) => self.dereference(*reference),
            Some(value_schema) => Some(value_schema),
            None => None,
        }
    }

    /// Reference positions.
    ///
    /// Should only be used when the schema is complete.
    pub fn reference_positions(&self) -> SchemaReferencePositions {
        SchemaReferencePositions::new(self)
    }

    /// Into validation.
    pub fn into_validation(self) -> Option<Expression<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        if !self.is_empty() {
            Some(Call::new_native("_schema".into(), vec![self.into()], floria::CallKind::Normal).into())
        } else {
            None
        }
    }
}

impl<AnnotatedT> From<ValueSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: ValueSchema<AnnotatedT>) -> Self {
        let mut value_schemas = BTreeMap::default();
        value_schemas.insert(SchemaReference::default(), schema);
        Self { value_schemas, data_type_references: Default::default() }
    }
}

impl<AnnotatedT> From<PrimitiveSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: PrimitiveSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<ScalarSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: ScalarSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<ListSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: ListSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<MapSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: MapSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<StructSchema<AnnotatedT>> for Schema<AnnotatedT> {
    fn from(schema: StructSchema<AnnotatedT>) -> Self {
        Self::from(ValueSchema::from(schema))
    }
}

impl<AnnotatedT> From<DataKind> for Schema<AnnotatedT> {
    fn from(data_kind: DataKind) -> Self {
        Self::from(ValueSchema::from(data_kind))
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for Schema<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        let positions = self.reference_positions();

        if self.value_schemas.len() == 1 {
            self.value_schemas.into_values().next().expect("has value schema").into_expression(&positions)
        } else {
            let list: Vec<_> =
                self.value_schemas.into_values().map(|value| value.into_expression(&positions)).collect();
            list.into()
        }
    }
}
