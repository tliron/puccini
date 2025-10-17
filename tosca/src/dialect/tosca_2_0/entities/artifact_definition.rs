use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    artifact_type::*,
    value_assignment::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// ArtifactDefinition
//

/// An artifact definition defines a named, typed file that can be associated with a node type or
/// node template and used by a TOSCA Orchestrator to facilitate deployment and implementation of
/// artifact operations.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// Puccini note: Though this is called a "definition" in the TOSCA spec, it is actually used both
/// as a definition and as a template. See
/// [ArtifactAssignment](super::artifact_assignment::ArtifactAssignment).
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory artifact type for the artifact definition.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The mandatory URI string (relative or absolute) that can be used to locate the artifact's
    /// file.
    #[resolve(required)]
    #[depict(style(string))]
    pub file: ByteString,

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

impl<AnnotatedT> ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Constructor.
    pub fn new_plugin(plugin: ByteString) -> Self {
        let mut floria_prefix = ValueAssignment::<AnnotatedT>::default();
        floria_prefix.expression = Some(plugin.into());

        let mut properties = ValueAssignments::default();
        properties.insert("floria-prefix".into(), floria_prefix);

        Self { type_name: Name::from(PLUGIN_ARTIFACT_TYPE).into(), properties, ..Default::default() }
    }

    /// Plugin file and prefix.
    pub fn plugin(&self) -> Result<(ByteString, Option<ByteString>), ToscaError<AnnotatedT>> {
        if self.type_name == Name::from(PLUGIN_ARTIFACT_TYPE).into() {
            let prefix = self
                .properties
                .get("floria-prefix")
                .and_then(|floria_prefix| floria_prefix.expression.as_ref())
                .and_then(|floria_prefix| {
                    if let Expression::Simple(floria_prefix) = floria_prefix { Some(floria_prefix) } else { None }
                })
                .and_then(|floria_prefix| {
                    if let Variant::Text(floria_prefix) = floria_prefix {
                        Some(floria_prefix.inner.clone())
                    } else {
                        None
                    }
                });

            Ok((self.file.clone(), prefix))
        } else {
            Err(WrongTypeError::new(
                "artifact definition".into(),
                self.type_name.to_string(),
                vec![PLUGIN_ARTIFACT_TYPE.into()],
            )
            .with_annotations_from_field(self, "type_name")
            .into())
        }
    }
}

impl<AnnotatedT> Subentity<Self> for ArtifactDefinition<AnnotatedT>
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
        complete_subentity_map_field!(property, properties, self, parent, parent_namespace, true, context);

        if let Some(parent) = parent {
            complete_none_field!(repository, self, parent);
            complete_none_field!(artifact_version, self, parent);
            complete_none_field!(checksum, self, parent);
            complete_none_field!(checksum_algorithm, self, parent);
        }

        let (artifact_type, artifact_type_namespace) =
            entity_from_full_name_field!(ARTIFACT_TYPE, ArtifactType, self, type_name, context);

        complete_subentity_map_field!(
            property,
            properties,
            self,
            artifact_type,
            artifact_type_namespace,
            true,
            context
        );

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            type_name: self.type_name.to_namespace(namespace),
            file: self.file.clone(),
            repository: self.repository.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            artifact_version: self.artifact_version.clone(),
            checksum: self.checksum.clone(),
            checksum_algorithm: self.checksum_algorithm.clone(),
            properties: self.properties.to_namespace(namespace),
            annotations: self.annotations.clone(),
        }
    }
}

//
// ArtifactDefinitions
//

/// Map of [ArtifactDefinition].
pub type ArtifactDefinitions<AnnotatedT> = BTreeMap<ByteString, ArtifactDefinition<AnnotatedT>>;
