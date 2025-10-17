use {
    floria_plugin_sdk::{data::*, utils::*, *},
    std::sync::*,
};

/// Error if argument is not true.
pub fn assert(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 1)?;
    let mut arguments = arguments.into_iter();

    set_assert_reason(None)?;

    let boolean = arguments.next().unwrap().must_evaluate(&call_site)?.cast_bool("argument")?;
    if boolean {
        Ok(None)
    } else {
        match get_assert_reason()? {
            Some(reason) => Err(format!("invalid because expression is false: |error|{}|", reason)),
            None => Err("invalid because an expression is false".into()),
        }
    }
}

/// Assert reason.
static ASSERT_REASON: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Default::default());

/// Get assert reason.
pub fn get_assert_reason() -> Result<Option<String>, String> {
    Ok(ASSERT_REASON.lock().map_escape_depiction_error()?.clone())
}

/// Set assert reason.
pub fn set_assert_reason(value: Option<String>) -> Result<(), String> {
    *ASSERT_REASON.lock().map_escape_depiction_error()? = value;
    Ok(())
}
