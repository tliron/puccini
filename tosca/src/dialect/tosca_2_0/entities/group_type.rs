use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    attribute_definition::*,
    property_definition::*,
};

use {
    compris::{annotate::*, depict::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
    std::collections::*,
};

/// As with most TOSCA entities, groups are typed. A group type definition is a type of TOSCA type
/// definition and as a result supports the common keynames listed in the section Common Keynames
/// in Type Definitions.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct GroupType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[depict(option, as(depict))]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[depict(option, as(depict))]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    ///	An optional map of property definitions for the group type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the group type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional list of one or more names of node types that are valid (allowed) as
    /// members of the group type.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub members: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(GroupType);

impl<AnnotatedT> Entity for GroupType<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion_state(&self) -> CompletionState {
        self.completion_state
    }

    fn complete(
        &mut self,
        derivation_path: &mut DerivationPath,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let (parent, parent_namespace) =
            completed_entity_checked_from_full_name_field!(GROUP_TYPE, self, derived_from, derivation_path, context);

        complete_subentity_map_field!(property, properties, self, parent, parent_namespace, false, context);
        complete_subentity_map_field!(attribute, attributes, self, parent, parent_namespace, false, context);
        complete_type_list_field!(members, self, parent, context);

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for GroupType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            derived_from: self.derived_from.to_namespace(namespace),
            version: self.version.clone(),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            properties: self.properties.to_namespace(namespace),
            attributes: self.attributes.to_namespace(namespace),
            members: self.members.to_namespace(namespace),
            annotations: self.annotations.clone(),
            completion_state: self.completion_state,
        }
    }
}

//
// GroupTypes
//

/// Map of [GroupType].
pub type GroupTypes<AnnotatedT> = BTreeMap<Name, GroupType<AnnotatedT>>;
