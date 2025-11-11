use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
    schema_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    smart_default::*,
    std::collections::*,
};

//
// PropertyDefinition
//

/// A property definition defines a named, typed value and related data that can be associated with
/// an entity defined in this specification (e.g., node types, relationship types, capability types,
/// etc.). Properties are used by template authors to provide configuration values to TOSCA entities
/// that indicate their desired state when they are instantiated. The value of a property can be
/// retrieved using the $get_property function within TOSCA service templates.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Depict, Resolve, SmartDefault)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory data type for the property.
    ///
    /// Puccini note: *Not* mandatory, as it can be inherited from parent.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for the property.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional key that declares a property as required (true) or not (false). Defaults to
    /// true.
    #[resolve]
    #[depict(option, style(name))]
    pub required: Option<bool>,

    /// An optional key that may provide a value to be used as a default if not provided by another
    /// means. The default keyname SHALL NOT be defined when property is not required (i.e. the
    /// value of the required keyname is false).
    #[resolve]
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// An optional key that may provide a fixed value to be used. A property that has a fixed
    /// value provided (as part of a definition or refinement) cannot be subject to a further
    /// refinement or assignment. That is, a fixed value cannot be changed.
    #[resolve]
    #[depict(option, as(depict))]
    pub value: Option<Expression<AnnotatedT>>,

    /// The optional validation clause for the property.
    #[resolve]
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,

    /// The schema definition for the keys used to identify entries in properties of type map (or
    /// types that derive from map). If not specified, the key_schema defaults to string. For
    /// properties of type other than map, the key_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub key_schema: Option<SchemaDefinition<AnnotatedT>>,

    /// The schema definition for the entries in properties of collection types such as list, map,
    /// or types that derive from list or map. If the property type is a collection type,
    /// entry_schema is mandatory. For other types, the entry_schema is not allowed.
    #[resolve]
    #[depict(option, as(depict))]
    pub entry_schema: Option<SchemaDefinition<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<Self> for PropertyDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_type_name_field!(self, parent, parent_namespace, true, context);
        complete_subentity_field!(key_schema, self, parent, parent_namespace, context);
        complete_subentity_field!(entry_schema, self, parent, parent_namespace, context);

        if let Some(parent) = parent {
            complete_optional_field!(required, self, parent);
            complete_namespaced_field!(default, self, parent, parent_namespace, context);
            complete_namespaced_field!(value, self, parent, parent_namespace, context);
            complete_validation!(self, parent);
        }

        let (data_type, _data_type_namespace) =
            completed_entity_from_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        if let Some(data_type) = data_type {
            complete_validation!(self, data_type);

            //let namespace = &self.type_name.namespace;

            // if requirement=true, default cannot be Some

            // if value=None, default cannot be Some

            // if "default" field is literal, we can check its type
            //
            // we can check if "key_schema" and "entry_schema" fields are allowed
            // (only for map and list types)
        }

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            type_name: self.type_name.to_namespace(namespace),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            required: self.required,
            default: self.default.to_namespace(namespace),
            value: self.value.to_namespace(namespace),
            validation: self.validation.to_namespace(namespace),
            key_schema: self.key_schema.to_namespace(namespace),
            entry_schema: self.entry_schema.to_namespace(namespace),
            annotations: self.annotations.clone(),
        }
    }
}

//
// PropertyDefinitions
//

/// Map of [PropertyDefinition].
pub type PropertyDefinitions<AnnotatedT> = BTreeMap<Name, PropertyDefinition<AnnotatedT>>;
