use super::super::super::super::grammar::*;

use {
    compris::{annotate::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// TriggerDefinition
//

/// A trigger definition defines an event, condition, action tuple associated with a policy.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct TriggerDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional description string for the trigger.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// The mandatory name of the event that activates the trigger's action.
    #[resolve(required)]
    #[depict(as(display), style(name))]
    pub event: ByteString,

    /// The optional condition that must evaluate to true in order for the trigger's action to be
    /// performed. Note: this is optional since sometimes the event occurrence itself is enough
    /// to trigger the action.
    #[resolve]
    #[depict(as(depict))]
    pub condition: Variant<AnnotatedT>,

    /// The list of sequential activities to be performed when the event is triggered, and the
    /// condition is met (i.e., evaluates to true).
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub action: Vec<Variant<AnnotatedT>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<Self> for TriggerDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        _parent: Option<&Self>,
        _parent_namespace: Option<&Namespace>,
        _context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        // TODO
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for TriggerDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, _namespace: Option<&Namespace>) -> Self {
        self.clone()
    }
}

//
// TriggerDefinitions
//

/// Map of [TriggerDefinition].
pub type TriggerDefinitions<AnnotatedT> = BTreeMap<ByteString, TriggerDefinition<AnnotatedT>>;
