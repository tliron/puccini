use super::super::super::entities::*;

use floria_plugin_sdk::{data::*, *};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $get_attribute function is used within a representation graph to obtain attribute values
/// from nodes and relationships that have been created from an application model described in a
/// service template. The nodes or relationships can be referenced by their name as assigned in the
/// service template or relative to the context where they are being invoked.
pub fn get_attribute(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    let call_site = call_site.entity()?;
    follow_tosca_path_to_property_value(call_site, &arguments, false).map(Some)
}
