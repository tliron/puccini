use {
    floria_plugin_sdk::{data::*, utils::*, *},
    puccini_plugin_sdk_tosca_2_0::data::*,
};

/// Coerce a value to a schema.
///
/// If the second argument is omitted, will use the call site value.
///
/// If the value is valid, meaning that it adheres to the schema, will return a canonicalized
/// version of the value, or [None] if already canonical. Otherwise, will return an error.
pub fn schema(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count_range(&arguments, 1, 2)?;
    let mut arguments = arguments.into_iter();

    let schema: Schema = arguments.next().unwrap().must_evaluate(&call_site)?.try_into()?;

    let value = match arguments.next() {
        Some(argument) => argument.must_evaluate(&call_site)?,
        None => call_site.value()?,
    };

    // TODO: None?
    Ok(Some(schema.coerce(value, &call_site)?))
}
