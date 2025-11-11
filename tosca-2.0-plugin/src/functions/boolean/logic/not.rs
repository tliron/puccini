use floria_plugin_sdk::{data::*, utils::*, *};

/// The $not function takes one Boolean argument. It evaluates to true if its argument evaluates to
/// false and evaluates to false if its argument evaluates to true.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn not(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 1)?;
    let mut arguments = arguments.into_iter();

    let argument = arguments.next().unwrap().must_evaluate(&call_site)?.cast_bool("argument")?;
    Ok(Some((!argument).into()))
}
