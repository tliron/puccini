use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
    schema::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
};

//
// SchemaDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// All entries in a list or map for one property or parameter must be of the same type. Similarly,
/// all keys for map entries for one property or parameter must be of the same type as well. A
/// TOSCA schema definition must be used to specify the type (for simple entries) or schema (for
/// complex entries) for keys and entries in TOSCA set types such as the TOSCA list or map.
///
/// If the schema definition specifies a map key, the type of the key schema must be derived
/// originally from the string type (which basically ensures that the schema type is a string with
/// additional validation clause). As there is little need for complex keys this caters to more
/// straight-forward and clear specifications. If the key schema is not defined it is assumed to be
/// string by default.
///
/// Schema definitions appear in data type definitions when derived_from a list or map type or in
/// parameter, property, or attribute definitions of a list or map type.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory data type for the key or entry. If this schema definition is for a map key,
    /// then the referred type must be derived originally from string.
    #[resolve(single, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for the schema.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// The optional validation clause that must evaluate to True for the property.
    #[resolve]
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// When the schema itself is of type map, the optional schema definition that is used to
    /// specify the type of the keys of that map's entries (if key_schema is not defined it is
    /// assumed to be "string" by default). For other schema types, the key_schema must not be
    /// defined.
    #[resolve]
    #[depict(option, as(depict))]
    pub key_schema: Option<Box<SchemaDefinition<AnnotatedT>>>,

    /// When the schema itself is of type list or map, the schema definition is mandatory and is
    /// used to specify the type of the entries in that map or list. For other schema types, the
    /// entry_schema must not be defined.
    #[resolve]
    #[depict(option, as(depict))]
    pub entry_schema: Option<Box<SchemaDefinition<AnnotatedT>>>,

    /// Data kind.
    #[depict(option, style(symbol))]
    pub data_kind: Option<DataKind>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Initialize a schema.
    pub fn initialize_schema(
        &self,
        schema: &mut Schema<AnnotatedT>,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<SchemaReference, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
    {
        let data_type = catalog.entity::<DataType<AnnotatedT>, _>(DATA_TYPE, &self.type_name, source_id)?;
        let reference = data_type.initialize_schema(&self.type_name, schema, self, source_id, catalog)?;
        Ok(reference.into())
    }
}

impl<AnnotatedT> Subentity<SchemaDefinition<AnnotatedT>> for SchemaDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        parent: Option<(&SchemaDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors_ref: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors_ref.to_error_recipient();

        if let Some((parent, _scope)) = &parent {
            if self.type_name.is_empty() && !parent.type_name.is_empty() {
                self.type_name = parent.type_name.clone();
            } else {
                validate_type_name(&self.type_name, &parent.type_name, catalog, errors)?;
            }

            complete_validation!(self, parent);

            if self.data_kind.is_none() && parent.data_kind.is_some() {
                self.data_kind = parent.data_kind;
            }
        }

        let data_type = completed_entity!(DATA_TYPE, DataType, self, type_name, catalog, source_id, errors);

        if let Some((data_type, _scope)) = data_type {
            complete_validation!(self, data_type);

            if self.data_kind.is_none() && data_type.data_kind.is_some() {
                self.data_kind = data_type.data_kind;
            }
        }

        complete_boxed_field!(key_schema, self, parent, catalog, source_id, errors_ref);
        complete_boxed_field!(entry_schema, self, parent, catalog, source_id, errors_ref);

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<SchemaDefinition<AnnotatedT>> for SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> Self {
        Self {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            description: self.description.clone(),
            validation: self.validation.clone(),
            key_schema: self
                .key_schema
                .as_ref()
                .and_then(|key_schema| Some(key_schema.convert_into_scope(scope).into())),
            entry_schema: self
                .entry_schema
                .as_ref()
                .and_then(|entry_schema| Some(entry_schema.convert_into_scope(scope).into())),
            data_kind: self.data_kind,
            annotations: self.annotations.clone(),
        }
    }
}

impl<AnnotatedT> SchemaDetails<AnnotatedT> for SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn default_expression(&self) -> Option<&Expression<AnnotatedT>> {
        None
    }

    fn key_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        self.key_schema.as_ref().map(|key_schema| key_schema.as_ref())
    }

    fn entry_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        self.entry_schema.as_ref().map(|entry_schema| entry_schema.as_ref())
    }

    fn validation(&self) -> Option<&Expression<AnnotatedT>> {
        self.validation.as_ref()
    }
}
