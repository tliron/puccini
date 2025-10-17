use floria_plugin_sdk::data::*;

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $get_artifact function is used to retrieve the location of artifacts defined by modelable
/// entities in a service template.
pub fn get_artifact(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(None)
}
