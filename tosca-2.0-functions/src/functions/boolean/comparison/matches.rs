use super::super::super::puccini::*;

use {
    floria_plugin_sdk::{data::*, utils::*},
    regex::*,
};

/// The $matches function takes two arguments. The first argument is a general string, and the
/// second argument is a string that encodes a regular expression pattern. It evaluates to true if
/// the first argument matches the regular expression pattern represented by the second argument
/// and false otherwise.
pub fn matches(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;

    let string = arguments.remove(0).must_evaluate(&call_site)?;
    let Expression::Text(string) = string else {
        return Err(format!("first argument must be a string: |error|{}|", string.type_name()));
    };

    let pattern = arguments.remove(0).must_evaluate(&call_site)?;
    let Expression::Text(pattern) = pattern else {
        return Err(format!("second argument must be a string: |error|{}|", pattern.type_name()));
    };
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
