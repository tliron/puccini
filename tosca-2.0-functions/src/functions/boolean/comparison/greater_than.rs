use super::super::super::{super::data::*, puccini::*};

use floria_plugin_sdk::{data::*, utils::*};

/// The $greater_than function takes two arguments of integer, float, string, timestamp, version,
/// any scalar type, or their derivations. It evaluates to true if both arguments are of the same
/// type, and if the first argument is greater than the second argument and evaluates to false
/// otherwise.
pub fn greater_than(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let left = arguments.next().unwrap().must_evaluate(&call_site)?;
    let right = arguments.next().unwrap().must_evaluate(&call_site)?;

    let left = left.coerce_if_custom(&right)?;
    let right = right.coerce_if_custom(&left)?;
    left.assert_same_type(&right, ">")?;

    let left = left.comparator()?;
    let right = right.comparator()?;

    Ok(Some(
        if left > right {
            true
        } else {
            set_assert_reason(Some(format!("{} > {}", left, right)))?;
            false
        }
        .into(),
    ))
}
