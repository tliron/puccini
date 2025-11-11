use {
    floria_plugin_sdk::{data::*, utils::*, *},
    puccini_plugin_sdk_tosca_2_0::entities::*,
};

/// The $get_input function is used to retrieve the values of parameters declared within the inputs
/// section of a TOSCA service template.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn get_input(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count_min(&arguments, 1)?;

    let path_site = call_site.entity()?.into_tosca_service()?.into();
    let mut parser = ToscaPathParser::new(&arguments);
    let input = parser.next_input(&path_site)?;
    parser.next_expression(&input).map(Some)
}
