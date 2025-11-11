use super::super::super::internal::*;

use {
    floria_plugin_sdk::{data::*, utils::*, *},
    puccini_plugin_sdk_tosca_2_0::data::*,
};

/// The $less_or_equal function takes two arguments of integer, float, string, timestamp, version,
/// any scalar type, or their derivations. It evaluates to true if both arguments are of the same
/// type, and if the first argument is less than or equal to the second argument and evaluates to
/// false otherwise
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn less_or_equal(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let left = arguments.next().unwrap().must_evaluate(&call_site)?;
    let right = arguments.next().unwrap().must_evaluate(&call_site)?;

    let left = left.coerce_if_custom(&right)?;
    let right = right.coerce_if_custom(&left)?;
    left.assert_same_type(&right, "≤")?;

    let left = left.comparator()?;
    let right = right.comparator()?;

    Ok(Some(
        if left <= right {
            true
        } else {
            set_assert_reason(Some(format!("{} ≤ {}", left, right)))?;
            false
        }
        .into(),
    ))
}
