use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
    schema_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// AttributeDefinition
//

/// An attribute definition defines a named, typed value that can be associated with an entity defined
/// in this specification (e.g., a node, relationship or capability type). Specifically, it is used
/// to expose the actual state of some a TOSCA entity after it has been deployed and instantiated (as
/// set by the TOSCA orchestrator).
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory data type for the attribute.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for the attribute.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional key that may provide a value to be used as a default if not provided by another
    /// means. This value SHALL be type compatible with the type declared by the attribute
    /// definition's type keyname.
    #[resolve]
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// The optional validation clause for the attribute.
    #[resolve]
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// The schema definition for the keys used to identify entries in attributes of type TOSCA map
    /// (or types that derive from map). If not specified, the key_schema defaults to string. For
    /// attributes of type other than map, the key_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub key_schema: Option<SchemaDefinition<AnnotatedT>>,

    /// The schema definition for the entries in attributes of collection types such as list, map,
    /// or types that derive from list or map) If the attribute type is a collection type,
    /// entry_schema is mandatory. For other types, the entry_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub entry_schema: Option<SchemaDefinition<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<Self> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_name_field!(type_name, self, parent, parent_namespace, context);
        complete_subentity_field!(key_schema, self, parent, parent_namespace, context);
        complete_subentity_field!(entry_schema, self, parent, parent_namespace, context);

        if let Some(parent) = parent {
            complete_none_field!(default, self, parent);
            complete_validation!(self, parent);
        }

        let (data_type, _data_type_namespace) =
            entity_from_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        if let Some(data_type) = data_type {
            complete_validation!(self, data_type);

            //let namespace = &self.type_name.namespace;

            // if "default" field is literal, we can check its type
            //
            // we can check if "key_schema" and "entry_schema" fields are allowed
            // (only for map and list types)
        }

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            type_name: self.type_name.to_namespace(namespace),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            default: self.default.clone(),
            validation: self.validation.clone(),
            key_schema: self.key_schema.as_ref().map(|schema_definition| schema_definition.to_namespace(namespace)),
            entry_schema: self.entry_schema.as_ref().map(|schema_definition| schema_definition.to_namespace(namespace)),
            annotations: self.annotations.clone(),
        }
    }
}

//
// AttributeDefinitions
//

/// Map of [AttributeDefinition].
pub type AttributeDefinitions<AnnotatedT> = BTreeMap<ByteString, AttributeDefinition<AnnotatedT>>;
