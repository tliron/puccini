use super::{instance::*, kind::*, vertex::*};

use floria_plugin_sdk::{entities::*, host};

//
// ToscaEdge
//

/// TOSCA edge.
pub trait ToscaEdge {
    /// TOSCA relationship's source node.
    fn tosca_source_node(&self) -> Result<Vertex, String>;

    /// TOSCA relationship's target capability.
    fn tosca_target_capability(&self) -> Result<Vertex, String>;

    /// TOSCA relationship's target node.
    fn tosca_target_node(&self) -> Result<Vertex, String>;

    /// TOSCA relationship's service.
    fn tosca_service(&self) -> Result<Vertex, String>;
}

impl ToscaEdge for Edge {
    fn tosca_source_node(&self) -> Result<Vertex, String> {
        self.assert_tosca(Some(ToscaKind::Relationship), None)?;
        let vertex: Vertex = host::get_entity(&self.source_vertex_id.clone().into())?.try_into()?;
        vertex.assert_tosca(Some(ToscaKind::Node), None)?;
        Ok(vertex)
    }

    fn tosca_target_capability(&self) -> Result<Vertex, String> {
        self.assert_tosca(Some(ToscaKind::Relationship), None)?;
        let vertex: Vertex = host::get_entity(&self.target_vertex_id.clone().into())?.try_into()?;
        vertex.assert_tosca(Some(ToscaKind::Capability), None)?;
        Ok(vertex)
    }

    fn tosca_target_node(&self) -> Result<Vertex, String> {
        let capability = self.tosca_target_capability()?;
        let vertex = capability.must_tosca_containing_node(ToscaKind::Capability, ToscaKind::Node)?;
        vertex.assert_tosca(Some(ToscaKind::Node), None)?;
        Ok(vertex)
    }

    fn tosca_service(&self) -> Result<Vertex, String> {
        self.tosca_source_node()?.tosca_service()
    }
}
