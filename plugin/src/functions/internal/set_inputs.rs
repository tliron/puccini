use {
    floria_plugin_sdk::{data::*, entities::*, utils::*, *},
    puccini_plugin_sdk_tosca_2_0::entities::*,
};

/// Event handler that sets service inputs.
pub fn set_inputs(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    if let Some(payload) = event_payload(arguments, &call_site)?
        && let Some(inputs) = tosca_inputs(&payload)
        && let Entity::Vertex(mut service) = call_site.entity()?
        && service.is_tosca(Some(ToscaKind::Service), None)
    {
        let mut modified = false;

        for (name, value) in &inputs.inner {
            if let Expression::Text(name) = name
                && let Some(property) = service.property_mut(&to_input_name(name))
            {
                property.updater = Some(value.clone().into());
                modified = true;
            } else {
                return Err(format!("undefined |meta|input|: |error|{}|", escape_depiction_markup(name)));
            }
        }

        if modified {
            // println!("{:?}", service.metadata.inner());
            host::add_entity(service.into())?;
        }
    }

    Ok(None)
}

fn to_input_name(name: &str) -> String {
    format!("input:{}", name)
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
