use super::{
    super::{super::super::grammar::*, dialect::*},
    interface_assignment::*,
    relationship_type::*,
    value_assignment::*,
};

use {
    compris::{annotate::*, depict::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
    std::collections::*,
};

//
// RelationshipTemplate
//

/// A relationship template specifies the occurrence of a relationship of a given type between
/// nodes in an application or service. A relationship template defines application-specific values
/// for the properties, relationships, or interfaces defined by its relationship type.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the relationship type on which the relationship template is based.
    ///
    /// Puccini note: *Not* mandatory, as it can be copied via "copy".
    #[resolve(key = "type")]
    #[depict(option, as(depict))]
    pub type_name: Option<FullName>,

    /// An optional description for the relationship template.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property assignments for the relationship template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute assignments for the relationship template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional map of interface assignments for the relationship template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceAssignments<AnnotatedT>,

    /// The optional (symbolic) name of another relationship template from which to copy all
    /// keynames and values into this relationship template.
    #[resolve]
    #[depict(option, as(depict))]
    pub copy: Option<Name>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl<AnnotatedT> Entity for RelationshipTemplate<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion_state(&self) -> CompletionState {
        self.completion_state
    }

    fn complete(
        &mut self,
        _derivation_path: &mut DerivationPath,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        if let Some(copy) =
            completed_entity_from_optional_name_field!(RELATIONSHIP_TEMPLATE, RelationshipTemplate, self, copy, context)
        {
            complete_optional_field!(type_name, self, copy);
            complete_optional_field!(description, self, copy);
            complete_field!(metadata, self, copy);
            complete_field!(properties, self, copy);
            complete_field!(attributes, self, copy);
            complete_field!(interfaces, self, copy);
        }

        if self.type_name.is_none() {
            context.problems.give(
                MissingRequiredKeyError::as_problem(Variant::<WithoutAnnotations>::from("type"))
                    .with_annotations_from(self),
            )?;
            return Ok(());
        }

        let (relationship_type, relationship_type_namespace) = completed_entity_from_optional_full_name_field!(
            RELATIONSHIP_TYPE,
            RelationshipType,
            self,
            type_name,
            context
        );

        complete_subentity_map_field!(
            property,
            properties,
            self,
            relationship_type,
            relationship_type_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            attribute,
            attributes,
            self,
            relationship_type,
            relationship_type_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            interface,
            interfaces,
            self,
            relationship_type,
            relationship_type_namespace,
            true,
            context
        );

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// RelationshipTemplates
//

/// Map of [RelationshipTemplate].
pub type RelationshipTemplates<AnnotatedT> = BTreeMap<Name, RelationshipTemplate<AnnotatedT>>;
