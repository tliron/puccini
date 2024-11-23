use floria_plugin_sdk::{data::*, utils::*};

/// The $xor function takes two Boolean arguments. It evaluates to false if both arguments either
/// evaluate to true or both arguments evaluate to false, and evaluates to true otherwise.
pub fn xor(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;

    let argument = arguments.remove(0).must_evaluate(&call_site)?;
    let Expression::Boolean(left) = argument else {
        return Err(format!("first argument not a |name|boolean|: |error|{}|", argument.type_name()));
    };

    let argument = arguments.remove(0).must_evaluate(&call_site)?;
    let Expression::Boolean(right) = argument else {
        return Err(format!("second argument not a |name|boolean|: |error|{}|", argument.type_name()));
    };

    Ok(Some((left != right).into()))
}
