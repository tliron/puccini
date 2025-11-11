use {
    floria_plugin_sdk::{data::*, *},
    puccini_plugin_sdk_tosca_2_0::data::*,
};

/// Returns the call site value (like [`$value`](super::super::graph::value)) while also optionally
/// applying a series of expressions to it.
///
/// Each expression's value is assigned to the call site value *in sequence*, allowing for complex
/// transformations and validations.
///
/// Value-less expressions will not affect the call site value, however they can still return an
/// error, for example if the value is invalid.
///
/// Expected to be called eagerly.
pub fn apply(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    reset_call_site_value()?;

    for preparer in arguments {
        // Value-less expressions will not affect the current call site value
        if let Some(expression) = preparer.evaluate(&call_site)? {
            set_call_site_value(expression.clone())?;
        }
    }

    Ok(Some(call_site.value()?))
}
