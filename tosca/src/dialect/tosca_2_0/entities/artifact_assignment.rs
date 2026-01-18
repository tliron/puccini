use super::{super::super::super::grammar::*, artifact_definition::*, value_assignment::*};

use {
    compris::{annotate::*, depict::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
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
    /// Puccini note: *Not* mandatory, as it can be inherited from definition.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The mandatory URI string (relative or absolute) that can be used to locate the artifact's
    /// file.
    ///
    /// Puccini note: *Not* mandatory, as it can be inherited from definition.
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

    /// Algorithm used to calculate the artifact checksum (e.g. MD5, SHA \[Ref\]). Shall be specified
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
        _name: Option<&Name>,
        artifact_definition: Option<&ArtifactDefinition<AnnotatedT>>,
        artifact_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        complete_type_name_field!(self, artifact_definition, artifact_definition_namespace, false, context);
        complete_subentity_map_field!(
            property,
            properties,
            self,
            artifact_definition,
            artifact_definition_namespace,
            true,
            context
        );

        if let Some(artifact_definition) = artifact_definition {
            complete_optional_field_to!(file, self, artifact_definition, || Some(artifact_definition.file.clone()));

            // if_none_call(
            //     &mut self.file,
            //     || Some(parent.file.clone()),
            //     &mut self.annotations,
            //     &parent.annotations,
            //     "file",
            // );

            complete_optional_field!(repository, self, artifact_definition);
            complete_optional_field!(artifact_version, self, artifact_definition);
            complete_optional_field!(checksum, self, artifact_definition);
            complete_optional_field!(checksum_algorithm, self, artifact_definition);
        }

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<ArtifactAssignment<AnnotatedT>> for ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> ArtifactAssignment<AnnotatedT> {
        ArtifactAssignment {
            type_name: self.type_name.to_namespace(namespace),
            file: Some(self.file.clone()),
            repository: self.repository.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            artifact_version: self.artifact_version.clone(),
            checksum: self.checksum.clone(),
            checksum_algorithm: self.checksum_algorithm.clone(),
            properties: self.properties.to_namespace(namespace),
            annotations: self.annotations.clone(), // same fields
        }
    }
}

//
// ArtifactAssignments
//

/// Map of [ArtifactAssignment].
pub type ArtifactAssignments<AnnotatedT> = BTreeMap<Name, ArtifactAssignment<AnnotatedT>>;
