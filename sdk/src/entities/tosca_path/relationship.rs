use super::super::super::entities::*;

use {
    floria_plugin_sdk::{data::*, entities::*},
    std::iter::*,
};

impl<'context> ToscaPathParser<'context> {
    /// Next site after relationship.
    pub fn next_site_after_relationship(&mut self, path_site: &Edge) -> Result<Option<Entity>, String> {
        Ok(match self.iterator.peek() {
            Some(argument) => match argument {
                Expression::Text(text) => match text.as_str() {
                    // SOURCE, <node_context>
                    "SOURCE" => {
                        self.iterator.next();
                        let node = path_site.tosca_source_node()?;
                        Some(match self.next_site_after_node(&node)? {
                            Some(site) => site,
                            None => node.into(),
                        })
                    }

                    // TARGET, <node_context>
                    "TARGET" => {
                        self.iterator.next();
                        let node = path_site.tosca_target_node()?;
                        Some(match self.next_site_after_node(&node)? {
                            Some(site) => site,
                            None => node.into(),
                        })
                    }

                    // CAPABILITY, RELATIONSHIP <idx>, <rel_context> | CAPABILITY
                    "CAPABILITY" => {
                        self.iterator.next();
                        let capability = path_site.tosca_target_capability()?;
                        Some(match self.next_site_after_capability(&capability)? {
                            Some(site) => site,
                            None => capability.into(),
                        })
                    }

                    _ => None,
                },

                _ => None,
            },

            None => None,
        })
    }
}
