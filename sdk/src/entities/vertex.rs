use super::{instance::*, kind::*, selector::*};

use floria_plugin_sdk::{entities::*, host, utils::escape_depiction_markup};

//
// ToscaVertex
//

/// TOSCA vertex.
pub trait ToscaVertex {
    /// TOSCA containing vertex.
    fn tosca_containing_node(&self) -> Result<Option<Vertex>, String>;

    /// TOSCA containing vertex.
    fn must_tosca_containing_node(&self, kind: ToscaKind, container_kind: ToscaKind) -> Result<Vertex, String>;

    /// TOSCA node's capability.
    fn tosca_capability(&self, capability_name: &str) -> Result<Option<Vertex>, String>;

    /// TOSCA node's outgoing relationship.
    fn tosca_outgoing_relationship(
        &self,
        requirement_name: &str,
        selector: ToscaSelector,
    ) -> Result<Option<Edge>, String>;

    /// TOSCA capability's incoming relationship.
    fn tosca_incoming_relationship(
        &self,
        requirement_name: &str,
        selector: ToscaSelector,
    ) -> Result<Option<Edge>, String>;

    /// TOSCA node or capability's service.
    fn tosca_service(&self) -> Result<Vertex, String>;

    /// TOSCA service's node.
    fn tosca_node(&self, node_template_name: &str, _selector: ToscaSelector) -> Result<Vertex, String>;
}

impl ToscaVertex for Vertex {
    fn tosca_containing_node(&self) -> Result<Option<Vertex>, String> {
        Ok(match &self.containing_vertex_id {
            Some(id) => Some(host::get_entity(&id.clone().into())?.try_into()?),
            None => None,
        })
    }

    fn must_tosca_containing_node(&self, kind: ToscaKind, container_kind: ToscaKind) -> Result<Vertex, String> {
        match self.tosca_containing_node()? {
            Some(node) => Ok(node),
            None => Err(format!(
                "TOSCA |meta|{}| |name|{}| not contained in |meta|{}|",
                kind.as_str(),
                escape_depiction_markup(self.id()),
                container_kind.as_str()
            )),
        }
    }

    fn tosca_capability(&self, capability_name: &str) -> Result<Option<Vertex>, String> {
        self.assert_tosca(Some(ToscaKind::Node), None)?;

        for id in &self.contained_vertex_ids {
            let vertex: Vertex = host::get_entity(&id.clone().into())?.try_into()?;
            if vertex.is_tosca(Some(ToscaKind::Capability), Some(capability_name)) {
                return Ok(Some(vertex));
            }
        }

        Ok(None)
    }

    fn tosca_outgoing_relationship(
        &self,
        requirement_name: &str,
        _selector: ToscaSelector,
    ) -> Result<Option<Edge>, String> {
        self.assert_tosca(Some(ToscaKind::Node), None)?;

        for id in &self.outgoing_edge_ids {
            let edge: Edge = host::get_entity(&id.clone().into())?.try_into()?;
            if edge.is_tosca(Some(ToscaKind::Relationship), Some(requirement_name)) {
                return Ok(Some(edge));
            }
        }

        Ok(None)
    }

    fn tosca_incoming_relationship(
        &self,
        requirement_name: &str,
        _selector: ToscaSelector,
    ) -> Result<Option<Edge>, String> {
        self.assert_tosca(Some(ToscaKind::Capability), None)?;

        for id in &self.outgoing_edge_ids {
            let edge: Edge = host::get_entity(&id.clone().into())?.try_into()?;
            if edge.is_tosca(Some(ToscaKind::Relationship), Some(requirement_name)) {
                return Ok(Some(edge));
            }
        }

        Ok(None)
    }

    fn tosca_service(&self) -> Result<Vertex, String> {
        if let Some(kind) = self.tosca_kind() {
            match kind {
                ToscaKind::Service => {
                    // TODO: this is silly; it would be more efficient to clone ourselves
                    match host::get_entity(&self.id.clone().into())? {
                        Entity::Vertex(vertex) => Ok(vertex),
                        _ => Err("entity is not a vertex".to_string()),
                    }
                }

                ToscaKind::Node => {
                    let node = self.must_tosca_containing_node(ToscaKind::Node, ToscaKind::Service)?;
                    node.assert_tosca(Some(ToscaKind::Service), None)?;
                    Ok(node)
                }

                ToscaKind::Capability => {
                    let node = self.must_tosca_containing_node(ToscaKind::Capability, ToscaKind::Node)?;
                    node.tosca_service()
                }

                ToscaKind::Interface => {
                    let node = self.must_tosca_containing_node(ToscaKind::Interface, ToscaKind::Node)?;
                    node.tosca_service()
                }

                _ => Err(format!(
                    "|name|{}| not a TOSCA |meta|{}|, |meta|{}|, |meta|{}|, or |meta|{}|",
                    escape_depiction_markup(self.id()),
                    ToscaKind::Service.as_str(),
                    ToscaKind::Node.as_str(),
                    ToscaKind::Capability.as_str(),
                    ToscaKind::Interface.as_str(),
                )),
            }
        } else {
            Err("not in TOSCA service".into())
        }
    }

    /// Find TOSCA node.
    fn tosca_node(&self, node_template_name: &str, _selector: ToscaSelector) -> Result<Vertex, String> {
        for id in &self.contained_vertex_ids {
            let vertex: Vertex = host::get_entity(&id.clone().into())?.try_into()?;
            if vertex.is_tosca(Some(ToscaKind::Node), Some(node_template_name)) {
                return Ok(vertex);
            }
        }

        Err("not found".into())
    }
}
