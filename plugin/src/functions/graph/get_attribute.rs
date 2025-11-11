use {
    floria_plugin_sdk::{data::*, *},
    puccini_plugin_sdk_tosca_2_0::entities::*,
};

/// The $get_attribute function is used within a representation graph to obtain attribute values
/// from nodes and relationships that have been created from an application model described in a
/// service template. The nodes or relationships can be referenced by their name as assigned in the
/// service template or relative to the context where they are being invoked.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn get_attribute(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    let path_site = call_site.entity()?;
    follow_tosca_path_to_property_value(path_site, &arguments, false).map(Some)
}
