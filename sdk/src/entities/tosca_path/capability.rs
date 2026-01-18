use super::super::super::entities::*;

use {
    floria_plugin_sdk::{data::*, entities::*, utils::*},
    std::iter::*,
};

impl<'context> ToscaPathParser<'context> {
    /// Next site after capability.
    pub fn next_site_after_capability(&mut self, path_site: &Vertex) -> Result<Option<Entity>, String> {
        match self.iterator.peek() {
            Some(argument) => match argument {
                Expression::Text(text) => match text.as_str() {
                    // RELATIONSHIP <idx>, <rel_context>
                    "RELATIONSHIP" => {
                        // Broken! See: https://github.com/oasis-tcs/tosca-specs/issues/315
                        self.iterator.next();
                        let selector = self.next_selector()?;
                        let requirement_name = "broken";
                        let relationship = path_site
                            .tosca_incoming_relationship(requirement_name, selector.clone())?
                            .ok_or_else(|| {
                            format!(
                                "TOSCA path: |meta|relationship| not found: |error|{} {}|",
                                escape_depiction_markup(requirement_name),
                                selector
                            )
                        })?;

                        Ok(Some(match self.next_site_after_relationship(&relationship)? {
                            Some(site) => site,
                            None => relationship.into(),
                        }))
                    }

                    _ => Ok(None),
                },

                _ => Ok(None),
            },

            None => Ok(None),
        }
    }
}
