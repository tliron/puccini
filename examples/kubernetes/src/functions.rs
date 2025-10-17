use floria_plugin_sdk::{data::*, utils::*, *};

/// Kubernetes.
pub fn kubernetes(arguments: Vec<Expression>, _call_site: CallSite) -> DispatchResult {
    assert_argument_count_min(&arguments, 1)?;
    Ok(None)
}
