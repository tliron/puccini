use floria_plugin_sdk::{data::*, utils::*};

/// The $not function takes one Boolean argument. It evaluates to true if its argument evaluates to
/// false and evaluates to false if its argument evaluates to true.
pub fn not(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 1)?;
    let mut arguments = arguments.into_iter();

    let argument = arguments.next().unwrap().must_evaluate(&call_site)?.cast_bool("argument")?;
    Ok(Some((!argument).into()))
}
