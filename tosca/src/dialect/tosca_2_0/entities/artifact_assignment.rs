use super::{super::super::super::grammar::*, artifact_definition::*, value::*};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

//
// ArtifactAssignment
//

/// See [ArtifactDefinition].
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ArtifactAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory artifact type for the artifact definition.
    ///
    /// Puccini note: Should not be mandatory for assignments.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The mandatory URI string (relative or absolute) that can be used to locate the artifact's
    /// file.
    ///
    /// Puccini note: Should not be mandatory for assignments.
    #[depict(option, style(string))]
    pub file: Option<ByteString>,

    /// The optional name of the repository definition that contains the location of the external
    /// repository that contains the artifact. The artifact is expected to be referenceable by its
    /// file URI within the repository.
    #[resolve]
    #[depict(option, as(depict))]
    pub repository: Option<Name>,

    /// The optional description for the artifact definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    ///	The version of this artifact. One use of this artifact_version is to declare the particular
    /// version of this artifact type, in addition to its mime_type (that is declared in the
    /// artifact type definition). Together with the mime_type it may be used to select a
    /// particular artifact processor for this artifact. For example, a Python interpreter that can
    /// interpret Python version 2.7.0.
    #[resolve]
    #[depict(option, style(string))]
    pub artifact_version: Option<ByteString>,

    /// The checksum used to validate the integrity of the artifact.
    #[resolve]
    #[depict(option, style(string))]
    pub checksum: Option<ByteString>,

    /// Algorithm used to calculate the artifact checksum (e.g. MD5, SHA [Ref]). Shall be specified
    /// if checksum is specified for an artifact.
    #[resolve]
    #[depict(option, style(string))]
    pub checksum_algorithm: Option<ByteString>,

    /// The optional map of property assignments associated with the artifact.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: ValueAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<ArtifactDefinition<AnnotatedT>> for ArtifactAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        artifact_definition: Option<(&ArtifactDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        complete_map_field!("property", properties, self, artifact_definition, catalog, source_id, errors);

        if let Some((artifact_definition, _scope)) = artifact_definition {
            validate_type_name(&self.type_name, &artifact_definition.type_name, catalog, errors)?;

            if_none_else!(file, self, artifact_definition, Some(artifact_definition.file.clone()));

            // if_none_call(
            //     &mut self.file,
            //     || Some(parent.file.clone()),
            //     &mut self.annotations,
            //     &parent.annotations,
            //     "file",
            // );

            if_none_clone!(repository, self, artifact_definition);
            if_none_clone!(artifact_version, self, artifact_definition);
            if_none_clone!(checksum, self, artifact_definition);
            if_none_clone!(checksum_algorithm, self, artifact_definition);
        }

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<ArtifactAssignment<AnnotatedT>> for ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> ArtifactAssignment<AnnotatedT> {
        ArtifactAssignment {
            type_name: self.type_name.clone().in_scope(scope.clone()),
            file: Some(self.file.clone()),
            repository: self.repository.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            artifact_version: self.artifact_version.clone(),
            checksum: self.checksum.clone(),
            checksum_algorithm: self.checksum_algorithm.clone(),
            properties: self.properties.convert_into_scope(scope),
            annotations: self.annotations.clone(), // same fields
        }
    }
}

//
// ArtifactAssignments
//

/// Map of [ArtifactAssignment].
pub type ArtifactAssignments<AnnotatedT> = BTreeMap<ByteString, ArtifactAssignment<AnnotatedT>>;
