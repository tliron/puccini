use super::super::client::*;

use floria_plugin_sdk::{data::*, utils::*, *};

/// Apply to Kubernetes.
pub fn apply_kubernetes(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 0)?;

    let entity = call_site.entity()?;
    let Some(configuration) = entity.property("client") else {
        return Err("client1".into());
    };
    let Some(configuration) = configuration.value() else {
        return Err("client2".into());
    };

    let configuration = configuration.cast_map("client")?.map();

    let client = KubernetesClient::new(&configuration)?;

    let deployments = client.get_deployments(Some("kube-system"))?;
    let deployments = to_json(&deployments).map_err(|error| escape_depiction_markup(error))?;

    Ok(Some(deployments.into()))
}
