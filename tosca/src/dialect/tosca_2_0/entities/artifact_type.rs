use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    property_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// ArtifactType
//

/// An artifact type is a reusable entity that defines the type of one or more files that are used
/// to define implementation or deployment artifacts that are referenced by nodes or relationships.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
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
    pub mime_type: Option<ByteString>,

    /// The optional file extension property for the artifact type.
    #[resolve]
    #[depict(option, iter(item), style(string))]
    pub file_ext: Option<Vec<ByteString>>,

    /// An optional map of property definitions for the artifact type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// True if internal.
    #[depict(style(symbol))]
    pub internal: bool,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(ArtifactType);

impl<AnnotatedT> ArtifactType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Constructor.
    pub fn new_internal() -> Self {
        Self { internal: true, ..Default::default() }
    }
}

impl<AnnotatedT> Entity for ArtifactType<AnnotatedT>
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
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let (parent, parent_namespace) =
            completed_entity_checked_from_full_name_field!(ARTIFACT_TYPE, self, derived_from, derivation_path, context);

        complete_subentity_map_field!(property, properties, self, parent, parent_namespace, false, context);

        if let Some(parent) = &parent {
            complete_optional_field!(mime_type, self, parent);
            complete_optional_field!(file_ext, self, parent);
        }

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for ArtifactType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            derived_from: self.derived_from.to_namespace(namespace),
            version: self.version.clone(),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            mime_type: self.mime_type.clone(),
            file_ext: self.file_ext.clone(),
            properties: self.properties.to_namespace(namespace),
            internal: self.internal,
            annotations: self.annotations.clone(),
            completion_state: self.completion_state,
        }
    }
}

//
// ArtifactTypes
//

/// Map of [ArtifactType].
pub type ArtifactTypes<AnnotatedT> = BTreeMap<Name, ArtifactType<AnnotatedT>>;
