use floria_plugin_sdk::{data::*, *};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $available_allocation function is used to retrieve the available allocation for capablity
/// properties that can be targeted by relationships to the capability. The main intended usage is
/// to use this function within the condition clause in a node_filter of a node with a select
/// directive; this allows to select only nodes that have a certain available capacity that for
/// example can accomodate the expected allocations when used as a target for a relationship.
pub fn available_allocation(_arguments: Vec<Expression>, _call_site: CallSite) -> DispatchResult {
    Ok(None)
}
