use floria_plugin_sdk::{data::*, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $and function takes two or more Boolean arguments. It evaluates to true if all its
/// arguments evaluate to true. It evaluates to false in all other cases.
pub fn and(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count_min(&arguments, 2)?;

    for argument in arguments {
        let argument = argument.must_evaluate(&call_site)?.cast_bool("argument")?;
        if !argument {
            return Ok(Some(false.into()));
        }
    }

    Ok(Some(true.into()))
}
