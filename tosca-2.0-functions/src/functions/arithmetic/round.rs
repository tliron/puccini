use floria_plugin_sdk::{data::*, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $round function takes a float argument. The result is an integer with the closest value to
/// the float argument. Equal value distance is rounded down (e.g. 3.5 is rounded down to 3, while
/// 3.53 is rounded up to 4).
pub fn round(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 1)?;

    // TODO: we round ties *away* from 0

    let argument = arguments.into_iter().next().unwrap().must_evaluate(&call_site)?.cast_f64("argument")?;
    Ok(Some(into_i64(argument.round())?.into()))
}
