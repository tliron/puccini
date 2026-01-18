use super::super::super::entities::*;

use {
    floria_plugin_sdk::{data::*, entities::*, utils::*},
    std::iter::*,
};

impl<'context> ToscaPathParser<'context> {
    /// Next site after node.
    pub fn next_site_after_node(&mut self, path_site: &Vertex) -> Result<Option<Entity>, String> {
        match self.iterator.peek() {
            Some(argument) => match argument {
                Expression::Text(text) => match text.as_str() {
                    // RELATIONSHIP, <requirement_name>, <idx>, <rel_context>
                    "RELATIONSHIP" => {
                        self.iterator.next();
                        let requirement_name =
                            self.iterator.next().ok_or_else(|| "TOSCA path: missing |meta|requirement| name")?;

                        match requirement_name {
                            Expression::Text(requirement_name) => {
                                let selector = self.next_selector()?;

                                let relationship = path_site
                                    .tosca_outgoing_relationship(requirement_name, selector.clone())?
                                    .ok_or_else(|| {
                                        format!(
                                            "TOSCA path: |meta|requirement| not found: |error|{} {}|",
                                            escape_depiction_markup(requirement_name),
                                            selector
                                        )
                                    })?;

                                Ok(Some(match self.next_site_after_relationship(&relationship)? {
                                    Some(site) => site,
                                    None => relationship.into(),
                                }))
                            }

                            _ => Err(format!(
                                "TOSCA path: |meta|requirement| name not |name|string|: |error|{}|",
                                requirement_name.type_name()
                            )),
                        }
                    }

                    // CAPABILITY, <capability_name>, RELATIONSHIP, <idx>, <rel_context> |
                    // CAPABILITY, <capability_name>
                    "CAPABILITY" => {
                        self.iterator.next();
                        let capability_name =
                            self.iterator.next().ok_or_else(|| "TOSCA path: missing |meta|capability| name")?;

                        match capability_name {
                            Expression::Text(capability_name) => {
                                let capability = path_site.tosca_capability(capability_name)?.ok_or_else(|| {
                                    format!(
                                        "TOSCA path: |meta|capability| not found: |error|{}|",
                                        escape_depiction_markup(capability_name)
                                    )
                                })?;

                                Ok(Some(match self.next_site_after_capability(&capability)? {
                                    Some(site) => site,
                                    None => capability.into(),
                                }))
                            }

                            _ => Err(format!(
                                "TOSCA path: |meta|capability| name not |name|string|: |error|{}|",
                                capability_name.type_name()
                            )),
                        }
                    }

                    _ => Ok(None),
                },

                _ => Ok(None),
            },

            None => Ok(None),
        }
    }
}
