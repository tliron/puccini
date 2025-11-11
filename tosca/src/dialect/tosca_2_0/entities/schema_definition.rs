use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
};

//
// SchemaDefinition
//

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
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory data type for the key or entry. If this schema definition is for a map key,
    /// then the referred type must be derived originally from string.
    ///
    /// Puccini note: *Not* mandatory, as it can be inherited from parent.
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

impl<AnnotatedT> Subentity<Self> for SchemaDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        parent: Option<&SchemaDefinition<AnnotatedT>>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_type_name_field!(self, parent, parent_namespace, true, context);
        complete_boxed_subentity_field!(key_schema, self, parent, parent_namespace, context);
        complete_boxed_subentity_field!(entry_schema, self, parent, parent_namespace, context);

        if let Some(parent) = parent {
            complete_validation!(self, parent);
            complete_optional_field!(data_kind, self, parent);
        }

        let (data_type, _data_type_namespace) =
            completed_entity_from_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        if let Some(data_type) = &data_type {
            complete_validation!(self, data_type);
            complete_optional_field!(data_kind, self, data_type);
        }

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            type_name: self.type_name.to_namespace(namespace),
            description: self.description.clone(),
            validation: self.validation.to_namespace(namespace),
            key_schema: self.key_schema.to_namespace(namespace),
            entry_schema: self.entry_schema.to_namespace(namespace),
            data_kind: self.data_kind,
            annotations: self.annotations.clone(),
        }
    }
}
