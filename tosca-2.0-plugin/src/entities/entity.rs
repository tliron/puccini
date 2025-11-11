use super::{edge::*, selector::*, vertex::*};

use floria_plugin_sdk::entities::*;

//
// ToscaEntity
//

/// TOSCA entity.
pub trait ToscaEntity {
    /// TOSCA node, capability, or relationship's service.
    fn tosca_service(&self) -> Result<Vertex, String>;

    /// TOSCA node.
    fn tosca_node(&self, node_template_name: &str, selector: ToscaSelector) -> Result<Vertex, String> {
        self.tosca_service()?.tosca_node(node_template_name, selector)
    }
}

impl ToscaEntity for Entity {
    fn tosca_service(&self) -> Result<Vertex, String> {
        match self {
            Entity::Vertex(vertex) => vertex.tosca_service(),
            Entity::Edge(edge) => edge.tosca_service(),
        }
    }
}
