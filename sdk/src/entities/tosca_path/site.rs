use super::super::super::entities::*;

use {
    floria_plugin_sdk::{data::*, entities::*},
    std::iter::*,
};

impl<'own> ToscaPathParser<'own> {
    /// Next site.
    pub fn next_site(&mut self, path_site: Entity) -> Result<Entity, String> {
        let argument = self.iterator.next().ok_or_else(|| "TOSCA path: empty")?;

        match argument {
            Expression::Text(text) => match text.as_str() {
                // SELF, <node_context> |
                // SELF, <rel_context>
                "SELF" => match path_site {
                    Entity::Vertex(ref vertex) => {
                        vertex.assert_tosca(Some(ToscaKind::Node), None)?;
                        Ok(self.next_site_after_node(vertex)?.unwrap_or(path_site))
                    }

                    Entity::Edge(ref edge) => {
                        edge.assert_tosca(Some(ToscaKind::Relationship), None)?;
                        Ok(self.next_site_after_relationship(edge)?.unwrap_or(path_site))
                    }
                },

                // <node_symbolic_name>, <idx>, <node_context> |
                // TODO? <relationship_symbolic_name>, <rel_context>
                name => {
                    let selector = self.next_selector()?;
                    let node = path_site.tosca_node(name, selector)?;
                    Ok(self.next_site_after_node(&node)?.unwrap_or_else(|| path_site))
                }
            },

            _ => Err(format!("TOSCA path: argument not |name|string|: |error|{}|", argument.type_name())),
        }
    }
}
