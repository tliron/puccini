use floria_plugin_sdk::{data::*, utils::*, *};

/// The $or function takes two or more Boolean arguments. It evaluates to false if all of its
/// arguments evaluate to false. It evaluates to true in all other cases.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn or(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count_min(&arguments, 2)?;

    for argument in arguments {
        let argument = argument.must_evaluate(&call_site)?.cast_bool("argument")?;
        if argument {
            return Ok(Some(true.into()));
        }
    }

    Ok(Some(false.into()))
}
