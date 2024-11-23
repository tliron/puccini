use super::super::super::data::*;

use floria_plugin_sdk::{data::*, utils::*};

/// This function is used as an argument inside validation functions. It returns the value of the
/// property, attribute, or parameter for which the validation clause is defined.
pub fn value(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 0)?;
    Ok(Some(call_site.value()?))
}
