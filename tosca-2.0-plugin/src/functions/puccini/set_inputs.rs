use super::super::super::entities::*;

use floria_plugin_sdk::{data::*, entities::*, utils::*, *};

/// Event handler that sets service inputs.
pub fn set_inputs(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    if let Some(payload) = event_payload(arguments, &call_site)?
        && let Some(inputs) = tosca_inputs(&payload)
        && let Entity::Vertex(service) = call_site.entity()?
        && service.is_tosca(Some(ToscaKind::Service), None)
    {
        for (name, value) in &inputs.inner {
            if let Expression::Text(name) = name
                && let Some(_property) = service.property(&format!("input:{}", name))
            {
                // to property updater
                println!("SETTING INPUT {}: {}", name, value);
            } else {
                println!("UNDEFINED {}: {}", name, value);
                return Err(format!("undefined |meta|input|: |error|{}|", escape_depiction_markup(name)));
            }
        }
    }

    Ok(None)
}

fn tosca_inputs(payload: &Map) -> Option<&Map> {
    if let Some(Expression::Map(tosca)) = payload.into_get("tosca")
        && let Some(Expression::Map(inputs)) = tosca.map().into_get("inputs")
    {
        Some(inputs.map())
    } else {
        None
    }
}
