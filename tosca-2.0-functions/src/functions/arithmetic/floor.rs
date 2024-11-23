use floria_plugin_sdk::{data::*, utils::*};

/// The $floor function takes a float argument. The result is an integer with the closest value
/// that is less or equal to the value of the float argument.
pub fn floor(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 1)?;

    let argument = arguments.into_iter().next().unwrap().must_evaluate(&call_site)?.cast_f64("argument")?;
    Ok(Some(into_i64(argument.floor())?.into()))
}
