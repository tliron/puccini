use {
    floria_plugin_sdk::{data::*, utils::*, *},
    puccini_plugin_sdk_tosca_2_0::data::*,
};

/// This function is used as an argument inside validation functions. It returns the value of the
/// property, attribute, or parameter for which the validation clause is defined.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn value(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 0)?;
    Ok(Some(call_site.value()?))
}
