use {
    floria_plugin_sdk::{data::*, utils::*},
    std::sync::*,
};

//
// CallSiteValue
//

/// Call site value.
pub trait CallSiteValue {
    /// Value.
    fn value(&self) -> Result<Expression, String>;
}

impl CallSiteValue for CallSite {
    fn value(&self) -> Result<Expression, String> {
        match value_stack_top()? {
            Some(value) => Ok(value),

            None => match self.property_value()? {
                Some(value) => Ok(value),
                None => Err("no call site value".into()),
            },
        }
    }
}

/// Reset call site value stack.
pub fn reset_call_site_value() -> Result<(), String> {
    set_value_stack(None)
}

/// Set call site value.
pub fn set_call_site_value(expression: Expression) -> Result<(), String> {
    set_value_stack(Some(vec![expression]))
}

/// Push to the top of the call site value stack.
pub fn push_call_site_value(expression: Expression) -> Result<(), String> {
    let mut stack = value_stack()?.unwrap_or_default();
    stack.push(expression);
    set_value_stack(Some(stack))
}

/// Remove the top of the call site value stack.
pub fn pop_call_site_value() -> Result<(), String> {
    if let Some(mut stack) = value_stack()? {
        stack.pop();
        set_value_stack(if !stack.is_empty() { Some(stack) } else { None })?;
    }
    Ok(())
}

// Utils

type Static<T> = LazyLock<Mutex<Option<T>>>;

static VALUE_STACK: Static<Vec<Expression>> = Static::new(|| Default::default());

fn set_value_stack(stack: Option<Vec<Expression>>) -> Result<(), String> {
    *VALUE_STACK.lock().map_escape_depiction_error()? = stack;
    Ok(())
}

fn value_stack() -> Result<Option<Vec<Expression>>, String> {
    Ok(VALUE_STACK.lock().map_escape_depiction_error()?.clone())
}

fn value_stack_top() -> Result<Option<Expression>, String> {
    Ok(value_stack()?.and_then(|stack| stack.last().cloned()))
}
