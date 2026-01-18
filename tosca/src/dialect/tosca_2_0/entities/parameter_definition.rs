use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
    schema_definition::*,
};

use {
    compris::{annotate::*, depict::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
    smart_default::*,
    std::collections::*,
};

//
// ParameterDefinition
//

// Copied from PropertyDefinition, except that "type" is not required

/// A parameter definition defines a named, typed value and related data that may be used to exchange
/// values between the TOSCA orchestrator and the external world.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Depict, Resolve, SmartDefault)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The data type of the parameter. While this keyname is mandatory for a TOSCA Property definition,
    /// it is not mandatory for a TOSCA parameter definition.
    #[resolve(key = "type")]
    #[depict(option, as(display), style(name))]
    pub type_name: Option<FullName>,

    /// The type-compatible value to assign to the parameter. Parameter values may be provided as
    /// the result from the evaluation of an expression or a function. May only be defined for
    /// outgoing parameters. Mutually exclusive with the mapping keyname.
    #[resolve]
    #[depict(option, as(depict))]
    pub value: Option<Expression<AnnotatedT>>,

    /// A mapping that specifies the node or relationship attribute into which the returned output
    /// value must be stored. May only be defined for incoming parameters. Mutually exclusive with
    /// the value keyname.
    #[resolve]
    #[depict(option, as(depict))]
    pub mapping: Option<Variant<AnnotatedT>>,

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

impl<AnnotatedT> Subentity<Self> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        complete_optional_parent_type_name_field!(type_name, self, parent, parent_namespace, false, context);
        complete_subentity_field!(key_schema, self, parent, parent_namespace, context);
        complete_subentity_field!(entry_schema, self, parent, parent_namespace, context);

        if self.value.is_some() && self.mapping.is_some() {
            // TODO: error
        }

        if let Some(parent) = parent {
            complete_optional_field!(required, self, parent);
            complete_namespaced_field!(default, self, parent, parent_namespace, context);
            complete_validation!(self, parent);
        }

        let (data_type, _data_type_namespace) =
            completed_entity_from_optional_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        if let Some(data_type) = data_type {
            complete_validation!(self, data_type);

            //let namespace = &self.type_name.namespace;

            // if "default" field is literal, we can check its type
            //
            // we can check if "key_schema" and "entry_schema" fields are allowed
            // (only for map and list types)

            // if let Some(validation) =
            //     give_unwrap!(data_type.schema_validation(self, None, context), context.problems, None)
            // {
            //     self.validation.join_apply(validation);
            // }
        }

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            type_name: self.type_name.to_namespace(namespace),
            value: self.value.to_namespace(namespace),
            mapping: self.mapping.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            required: self.required,
            default: self.default.to_namespace(namespace),
            validation: self.validation.to_namespace(namespace),
            key_schema: self.key_schema.to_namespace(namespace),
            entry_schema: self.entry_schema.to_namespace(namespace),
            annotations: self.annotations.clone(),
        }
    }
}

//
// ParameterDefinitions
//

/// Map of [ParameterDefinition].
pub type ParameterDefinitions<AnnotatedT> = BTreeMap<Name, ParameterDefinition<AnnotatedT>>;
