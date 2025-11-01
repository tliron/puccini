use super::{
    super::{super::super::grammar::*, dialect::*},
    interface_assignment::*,
    relationship_type::*,
    value::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

//
// RelationshipTemplate
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A relationship template specifies the occurrence of a relationship of a given type between
/// nodes in an application or service. A relationship template defines application-specific values
/// for the properties, relationships, or interfaces defined by its relationship type.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the relationship type on which the relationship template is based.
    #[resolve(required, key = "type")]
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
        catalog: &mut Catalog,
        source_id: &SourceID,
        _derivation_path: &mut DerivationPath,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let errors = &mut errors.to_error_recipient();

        if let Some(copy) = &self.copy {
            let Some(copy) = catalog.completed_entity::<RelationshipTemplate<AnnotatedT>, _, _>(
                RELATIONSHIP_TEMPLATE,
                &copy.clone().into(),
                source_id,
                errors,
            )?
            else {
                return Ok(());
            };

            complete_field_none!(type_name, self, copy);
            complete_field_none!(description, self, copy);
            complete_field_empty!(metadata, self, copy);
            complete_field_empty!(properties, self, copy);
            complete_field_empty!(attributes, self, copy);
            complete_field_empty!(interfaces, self, copy);
        }

        if self.type_name.is_none() {
            errors.give(MissingRequiredError::new("relationship type name".into(), Some("type_name".into())))?;
            return Ok(());
        }

        let (relationship_type, type_scope) = entity_from_name_field_option!(
            RELATIONSHIP_TYPE,
            RelationshipType,
            self,
            type_name,
            catalog,
            source_id,
            errors
        );

        complete_subentity_map_field!(
            property,
            properties,
            type_scope,
            self,
            relationship_type,
            true,
            catalog,
            source_id,
            errors
        );
        complete_subentity_map_field!(
            attribute,
            attributes,
            type_scope,
            self,
            relationship_type,
            true,
            catalog,
            source_id,
            errors
        );
        complete_subentity_map_field!(
            interface,
            interfaces,
            type_scope,
            self,
            relationship_type,
            true,
            catalog,
            source_id,
            errors
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
