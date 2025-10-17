use floria_plugin_sdk::data::*;

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// This function is used to return the runtime index of the current relationship in the list of
/// relationships created from the same requirement. The first index is 0. The function should not
/// be used outside a valid relationship context (i.e. a relationship type definitiom, or a
/// requirement definition or assignment).
pub fn relationship_index(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(None)
}
