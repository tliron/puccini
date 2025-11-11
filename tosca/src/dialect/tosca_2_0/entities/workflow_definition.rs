use super::super::super::super::grammar::*;

use {compris::resolve::*, depiction::*, std::collections::*};

//
// WorkflowDefinition
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// TODO
#[derive(Clone, Debug, Default, Depict, Resolve)]
pub struct WorkflowDefinition {}

//
// WorkflowDefinitions
//

/// Map of [WorkflowDefinition].
pub type WorkflowDefinitions = BTreeMap<Name, WorkflowDefinition>;
