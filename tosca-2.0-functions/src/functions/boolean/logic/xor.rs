use floria_plugin_sdk::{data::*, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $xor function takes two Boolean arguments. It evaluates to false if both arguments either
/// evaluate to true or both arguments evaluate to false, and evaluates to true otherwise.
pub fn xor(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let left = arguments.next().unwrap().must_evaluate(&call_site)?.cast_bool("first argument")?;
    let right = arguments.next().unwrap().must_evaluate(&call_site)?.cast_bool("second argument")?;

    Ok(Some((left != right).into()))
}
