use super::{
    super::super::super::grammar::*, group_template::*, node_template::*, parameter_definition::*, policy_template::*,
    relationship_template::*, workflow_definition::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
};

//
// ServiceTemplate
//

/// This section defines the service template of a TOSCA file. The main ingredients of the service
/// template are node templates representing components of the application and relationship
/// templates representing links between the components. These elements are defined in the nested
/// node_templates section and the nested relationship_templates sections, respectively.
/// Furthermore, a service template allows for defining input parameters, output parameters,
/// workflows as well as grouping of node templates and associated policies.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct ServiceTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional description for the service template.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information about this service template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of input parameters (i.e., as parameter definitions) for the service
    /// template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub inputs: ParameterDefinitions<AnnotatedT>,

    /// An optional map of output parameters (i.e., as parameter definitions) for the service
    /// template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub outputs: ParameterDefinitions<AnnotatedT>,

    /// A mandatory map of node template definitions for the service template.
    #[resolve(required)]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub node_templates: NodeTemplates<AnnotatedT>,

    /// An optional map of relationship templates for the service template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub relationship_templates: RelationshipTemplates<AnnotatedT>,

    /// An optional map of group definitions whose members are node templates defined within
    /// this same service template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub groups: GroupTemplates<AnnotatedT>,

    /// An optional list of policy definitions for the service template.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub policies: PolicyTemplates<AnnotatedT>,

    /// An optional declaration that exports the service template as an implementation of a Node
    /// type. This also includes the mappings between the external node type's capabilities and
    /// requirements to existing implementations of those capabilities and requirements on node
    /// templates declared within the service template.
    #[resolve]
    #[depict(option, as(depict))]
    pub substitution_mappings: Option<Variant<AnnotatedT>>,

    /// An optional map of workflow definitions for the service template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub workflows: WorkflowDefinitions,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl<AnnotatedT> Entity for ServiceTemplate<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion_state(&self) -> CompletionState {
        self.completion_state
    }

    fn complete(
        &mut self,
        _derivation_path: &mut DerivationPath,
        _context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);

        // TODO: inputs and outputs

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}
