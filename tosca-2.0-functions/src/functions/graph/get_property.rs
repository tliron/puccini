use super::super::super::entities::*;

use floria_plugin_sdk::{data::*, *};

/// The $get_property function is used to retrieve property values of modelable entities in the
/// representation graph. Note that the get_property function may only retrieve the static values
/// of parameter or property definitions of a TOSCA application as defined in the TOSCA service
/// template. The $get_attribute function should be used to retrieve values for attribute
/// definitions (or property definitions reflected as attribute definitions) from the
/// representation graph of the TOSCA application (as realized by the TOSCA orchestrator).
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn get_property(arguments: Vec<Expression>, site: CallSite) -> DispatchResult {
    let call_site = site.entity()?;
    follow_tosca_path_to_property_value(call_site, &arguments, true).map(Some)
}
