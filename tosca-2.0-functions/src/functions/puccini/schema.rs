use super::super::super::data::*;

use floria_plugin_sdk::{data::*, utils::*};

/// Coerce a value to a schema.
///
/// If the second argument is omitted, will use the call site value.
///
/// If the value is valid, meaning that it adheres to the schema, will return a canonicalized
/// version of the value, or [None] if already canonical. Otherwise, will return an error.
pub fn schema(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count_range(&arguments, 1, 2)?;

    let schema: Schema = arguments.remove(0).must_evaluate(&call_site)?.try_into()?;

    let value = if arguments.is_empty() {
        call_site.value()?
    } else {
        arguments.remove(0).must_evaluate(&call_site)?
    };

    // TODO: None?
    Ok(Some(schema.coerce(value, &call_site)?))
}
