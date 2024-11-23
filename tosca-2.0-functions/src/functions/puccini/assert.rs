use {
    floria_plugin_sdk::{data::*, utils::*},
    std::sync::*,
};

/// Error if argument is not true.
pub fn assert(mut arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 1)?;

    set_assert_reason(None)?;

    if let Expression::Boolean(boolean) = arguments.remove(0).must_evaluate(&call_site)?
        && boolean
    {
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
