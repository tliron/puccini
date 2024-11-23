use super::super::super::{super::data::*, puccini::*};

use floria_plugin_sdk::{data::*, utils::*};

/// The $equal function takes two arguments that have the same type. It evaluates to true if the
/// arguments are equal. An $equal function that uses arguments of different types SHOULD be
/// flagged as an error.
pub fn equal(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let left = arguments.next().unwrap().must_evaluate(&call_site)?;
    let right = arguments.next().unwrap().must_evaluate(&call_site)?;

    let left = left.coerce_if_custom(&right)?;
    let right = right.coerce_if_custom(&left)?;
    left.assert_same_type(&right, "=")?;

    // Note: equal can work directly on any type (we don't need comparators)

    Ok(Some(
        if left == right {
            true
        } else {
            set_assert_reason(Some(format!("{} = {}", left, right)))?;
            false
        }
        .into(),
    ))
}
