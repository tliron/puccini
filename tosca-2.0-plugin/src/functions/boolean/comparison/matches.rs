use super::super::super::puccini::*;

use {
    floria_plugin_sdk::{data::*, utils::*, *},
    regex::*,
};

/// The $matches function takes two arguments. The first argument is a general string, and the
/// second argument is a string that encodes a regular expression pattern. It evaluates to true if
/// the first argument matches the regular expression pattern represented by the second argument
/// and false otherwise.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn matches(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let string = arguments.next().unwrap().must_evaluate(&call_site)?.cast_string("first argument")?;
    let pattern = arguments.next().unwrap().must_evaluate(&call_site)?.cast_string("second argument")?;
    let regex = Regex::new(&pattern).map_escape_depiction_error()?;

    Ok(Some(
        if regex.is_match(&string) {
            true
        } else {
            set_assert_reason(Some(format!("{} =~ {}", string, pattern)))?;
            false
        }
        .into(),
    ))
}
