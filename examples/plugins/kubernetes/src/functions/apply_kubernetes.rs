use super::{super::client::*, krm::*};

use {
    floria_plugin_sdk::{data::*, entities::*, utils::*, *},
    puccini_plugin_sdk_tosca_2_0::entities::*,
};

/// Apply to Kubernetes.
pub fn apply_kubernetes(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 0)?;

    let entity = call_site.entity()?;
    let configuration = entity.must_property("kubernetes-client")?.must_value("kubernetes-client")?;
    let configuration = configuration.cast_map("kubernetes-client")?.map();

    let client = KubernetesClient::new(&configuration)?;

    let deployment_class_id = Id {
        kind: EntityKind::Class,
        directory: Default::default(),
        name: "capability-type:boutique:ms:krm:Deployment".into(),
    };

    let (node, capabilities) = get_capabilities(call_site.entity()?, &deployment_class_id)?;

    let label = node.property("label").and_then(|label| label.value()).unwrap_or("unknown".into());
    let label = label.cast_string("label")?;

    for capability in capabilities {
        if let Some(containers) = capability.property("containers")
            && let Some(_containers) = containers.value()
        {
            //log!("apply_kubernetes", "{}", containers);
            let deployment = new_deployment("app", &label);
            client.create_deployment("boutique", deployment)?;
        }
    }

    Ok(None)
}

fn get_capabilities(entity: Entity, class_id: &Id) -> Result<(Vertex, Vec<Vertex>), DispatchError> {
    let mut capabilities = Vec::default();

    let Entity::Vertex(interface) = entity else {
        return Err("not a vertex".into());
    };

    interface.assert_tosca(Some(ToscaKind::Interface), None)?;

    let node = interface.must_tosca_containing_node(ToscaKind::Interface, ToscaKind::Node)?;

    for id in &node.contained_vertex_ids {
        let vertex: Vertex = host::get_entity(&id.clone().into())?.try_into()?;
        if vertex.is_tosca(Some(ToscaKind::Capability), None) {
            if vertex.has_class_id(class_id) {
                capabilities.push(vertex);
            }
        }
    }

    Ok((node, capabilities))
}
