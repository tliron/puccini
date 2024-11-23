use floria_plugin_sdk::{data::*, utils::*};

/// The $not function takes one Boolean argument. It evaluates to true if its argument evaluates to
/// false and evaluates to false if its argument evaluates to true.
pub fn not(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 1)?;

    let argument = arguments.remove(0).must_evaluate(&call_site)?;
    let Expression::Boolean(value) = argument else {
        return Err(format!("argument not a |name|boolean|: |error|{}|", argument.type_name()));
    };

    Ok(Some((!value).into()))
}
