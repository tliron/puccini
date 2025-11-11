use super::{edge::*, instance::*, kind::*, selector::*, vertex::*};

use floria_plugin_sdk::entities::*;

//
// ToscaEntity
//

/// TOSCA entity.
pub trait ToscaEntity {
    /// The TOSCA service vertex to which this entity belongs.
    fn tosca_service(&self) -> Result<Vertex, String>;

    /// The TOSCA service vertex to which this entity belongs.
    fn into_tosca_service(self) -> Result<Vertex, String>;

    /// A TOSCA node in the service to which this entity belongs.
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

    fn into_tosca_service(self) -> Result<Vertex, String> {
        match self {
            Entity::Vertex(vertex) => {
                if vertex.is_tosca(Some(ToscaKind::Service), None) {
                    Ok(vertex)
                } else {
                    vertex.tosca_service()
                }
            }
            Entity::Edge(edge) => edge.tosca_service(),
        }
    }
}
