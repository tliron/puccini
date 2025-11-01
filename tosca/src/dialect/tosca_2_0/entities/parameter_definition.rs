use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
    schema_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    smart_default::*,
    std::collections::*,
};

//
// ParameterDefinition
//

// Copied from PropertyDefinition, except that "type" is not required

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A parameter definition defines a named, typed value and related data that may be used to exchange
/// values between the TOSCA orchestrator and the external world.
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

impl<AnnotatedT> Subentity<ParameterDefinition<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        scope: Option<&Scope>,
        parent: Option<&Self>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors_ref: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors_ref.to_error_recipient();

        //complete_type_name_value!(type_name, scope, self, parent, catalog, errors);

        if let Some(parent) = parent {
            complete_field_none!(required, self, parent);
            complete_field_none!(default, self, parent);
            complete_validation!(self, parent);
        }

        let (data_type, _type_scope) =
            entity_from_name_field_option!(DATA_TYPE, DataType, self, type_name, catalog, source_id, errors);

        if let Some(data_type) = data_type {
            complete_validation!(self, data_type);

            //let scope = &self.type_name.scope;

            // if "default" field is literal, we can check its type
            //
            // we can check if "key_schema" and "entry_schema" fields are allowed
            // (only for map and list types)
        }

        if let Some(type_name) = &self.type_name {
            if let Some(parent) = parent
                && let Some(parent_type_name) = &parent.type_name
            {
                validate_type_name(type_name, parent_type_name, catalog, errors)?;
            }
        }

        complete_subentity_field!(key_schema, scope, self, parent, catalog, source_id, errors_ref);
        complete_subentity_field!(entry_schema, scope, self, parent, catalog, source_id, errors_ref);

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<ParameterDefinition<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: Option<&Scope>) -> Self {
        Self {
            type_name: self.type_name.into_scoped(scope),
            value: self.value.clone(),
            mapping: self.mapping.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            required: self.required,
            default: self.default.clone(),
            validation: self.validation.clone(),
            key_schema: self.key_schema.as_ref().map(|schema_definition| schema_definition.into_scoped(scope)),
            entry_schema: self.entry_schema.as_ref().map(|schema_definition| schema_definition.into_scoped(scope)),
            annotations: self.annotations.clone(),
        }
    }
}

//
// ParameterDefinitions
//

/// Map of [ParameterDefinition].
pub type ParameterDefinitions<AnnotatedT> = BTreeMap<ByteString, ParameterDefinition<AnnotatedT>>;
