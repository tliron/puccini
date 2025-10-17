use floria_plugin_sdk::data::*;

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $get_input function is used to retrieve the values of parameters declared within the inputs
/// section of a TOSCA service template.
pub fn get_input(_arguments: Vec<Expression>, _call_site: CallSite) -> Result<Option<Expression>, String> {
    Ok(Some(123.into()))
}
