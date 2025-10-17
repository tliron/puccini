use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    property_definition::*,
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
// ArtifactType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An artifact type is a reusable entity that defines the type of one or more files that are used
/// to define implementation or deployment artifacts that are referenced by nodes or relationships.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ArtifactType<AnnotatedT>
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

    /// The optional mime type property for the artifact type.
    #[resolve]
    #[depict(option, style(string))]
    pub mime_type: Option<Annotate<ByteString, AnnotatedT>>,

    /// The optional file extension property for the artifact type.
    #[resolve]
    #[depict(option, iter(item), style(string))]
    pub file_ext: Option<Vec<ByteString>>,

    /// An optional map of property definitions for the artifact type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(ArtifactType);

impl<AnnotatedT> Entity for ArtifactType<AnnotatedT>
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
        derivation_path: &mut DerivationPath,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let errors = &mut errors.to_error_recipient();

        let (parent, parent_scope) = entity_from_name_field_checked!(
            ARTIFACT_TYPE,
            self,
            derived_from,
            catalog,
            source_id,
            derivation_path,
            errors
        );

        complete_subentity_map_field!(
            property,
            properties,
            parent_scope,
            self,
            parent,
            false,
            catalog,
            source_id,
            errors
        );

        if let Some(parent) = parent {
            complete_field_none!(mime_type, self, parent);
            complete_field_none!(file_ext, self, parent);
        }

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// ArtifactTypes
//

/// Map of [ArtifactType].
pub type ArtifactTypes<AnnotatedT> = BTreeMap<Name, ArtifactType<AnnotatedT>>;
