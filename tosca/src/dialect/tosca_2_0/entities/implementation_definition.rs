use super::{super::super::super::grammar::*, artifact_definition::*};

use {
    compris::{annotate::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::{error::*, immutable::*},
};

//
// ImplementationDefinition
//

/// An operation implementation definition specifies one or more artifacts (e.g. scripts) to be
/// used as the implementation for an operation in an interface.
///
/// A notification implementation definition specifies one or more artifacts to be used by the
/// orchestrator to subscribe and receive a particular notification (i.e. the artifact implements
/// the notification).
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ImplementationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional implementation artifact (i.e., the primary script file within a TOSCA CSAR
    /// file).
    #[resolve(single)]
    #[depict(option, as(depict))]
    pub primary: Option<ImplementationDefinitionArtifact<AnnotatedT>>,

    /// The optional list of one or more dependent or secondary implementation artifacts which are
    /// referenced by the primary implementation artifact (e.g., a library the script installs or
    /// a secondary script).
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub dependencies: Vec<ImplementationDefinitionArtifact<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> ImplementationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Constructor.
    pub fn new_plugin(plugin: ByteString) -> Self {
        Self { primary: Some(ImplementationDefinitionArtifact::new_plugin(plugin)), ..Default::default() }
    }

    /// Plugin file and prefix.
    pub fn plugin(&self) -> Result<Option<(ByteString, Option<ByteString>)>, ToscaError<AnnotatedT>> {
        Ok(match &self.primary {
            Some(artifact) => match artifact {
                ImplementationDefinitionArtifact::Definition(artifact_definition) => {
                    Some(artifact_definition.plugin()?)
                }
                ImplementationDefinitionArtifact::Name(_) => None,
            },
            None => None,
        })
    }
}

impl<AnnotatedT> Subentity<Self> for ImplementationDefinition<AnnotatedT>
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
        complete_subentity_field!(primary, self, parent, parent_namespace, context);

        // TODO: dependencies

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for ImplementationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            primary: self.primary.to_namespace(namespace),
            dependencies: self.dependencies.to_namespace(namespace),
            annotations: self.annotations.clone(),
        }
    }
}

//
// ImplementationDefinitionArtifact
//

/// [ImplementationDefinition] artifact.
#[derive(Clone, Debug, Depict)]
pub enum ImplementationDefinitionArtifact<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Definition.
    #[depict(as(depict))]
    Definition(ArtifactDefinition<AnnotatedT>),

    /// Name.
    #[depict(as(depict))]
    Name(Name),
}

impl<AnnotatedT> ImplementationDefinitionArtifact<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Constructor.
    pub fn new_plugin(plugin: ByteString) -> Self {
        Self::Definition(ArtifactDefinition::new_plugin(plugin))
    }
}

impl<AnnotatedT> Subentity<Self> for ImplementationDefinitionArtifact<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<ByteString>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        match self {
            Self::Definition(definition) => {
                let parent = parent.and_then(|parent| match parent {
                    Self::Definition(definition) => Some(definition),
                    Self::Name(_) => None,
                });
                definition.complete(name, parent, parent_namespace, context)
            }

            // turn names into definitions
            Self::Name(_name) => todo!(),
        }
    }
}

impl<AnnotatedT> ToNamespace<Self> for ImplementationDefinitionArtifact<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        match self {
            Self::Definition(artifact_definition) => Self::Definition(artifact_definition.to_namespace(namespace)),
            Self::Name(name) => Self::Name(name.clone()),
        }
    }
}

impl<AnnotatedT> Resolve<ImplementationDefinitionArtifact<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorReceiverT>(
        self,
        errors: &mut ErrorReceiverT,
    ) -> ResolveResult<ImplementationDefinitionArtifact<AnnotatedT>, AnnotatedT>
    where
        ErrorReceiverT: ErrorReceiver<ResolveError<AnnotatedT>>,
    {
        Ok(match self {
            Self::Text(text) => Variant::from(text)
                .resolve_with_errors(errors)?
                .and_then(|name| Some(ImplementationDefinitionArtifact::Name(name))),

            Self::Map(map) => Variant::from(map).resolve_with_errors(errors)?.and_then(|artifact_definition| {
                Some(ImplementationDefinitionArtifact::Definition(artifact_definition))
            }),

            _ => {
                errors.give(
                    IncompatibleVariantTypeError::new_from(&self, &["text", "map"]).with_annotations_from(&self),
                )?;
                None
            }
        })
    }
}
