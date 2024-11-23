use super::super::super::super::data::*;

use {
    floria_plugin_sdk::{data::*, utils::*},
    std::{iter::*, slice::*},
};

//
// ToscaPathParser
//

/// TOSCA Path parser.
pub struct ToscaPathParser<'own> {
    iterator: Peekable<Iter<'own, Expression>>,
}

impl<'own> ToscaPathParser<'own> {
    /// Constructor.
    pub fn new(arguments: &'own Vec<Expression>) -> Self {
        Self { iterator: arguments.iter().peekable() }
    }

    /// Next site.
    pub fn next_site(&mut self, path_site: Entity) -> Result<Entity, String> {
        let argument = self.iterator.next().ok_or_else(|| "invalid TOSCA path: empty")?;

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

            _ => Err(format!("invalid TOSCA path: argument not a string: {}", argument.type_name())),
        }
    }

    /// Next site after node.
    pub fn next_site_after_node(&mut self, path_site: &Vertex) -> Result<Option<Entity>, String> {
        match self.iterator.peek() {
            Some(argument) => match argument {
                Expression::Text(text) => match text.as_str() {
                    // RELATIONSHIP, <requirement_name>, <idx>, <rel_context>
                    "RELATIONSHIP" => {
                        self.iterator.next();
                        let requirement_name =
                            self.iterator.next().ok_or_else(|| "invalid TOSCA path: missing requirement name")?;

                        match requirement_name {
                            Expression::Text(requirement_name) => {
                                let selector = self.next_selector()?;

                                let relationship = path_site
                                    .tosca_outgoing_relationship(requirement_name, selector.clone())?
                                    .ok_or_else(|| {
                                        format!("TOSCA path: requirement not found: {} {}", requirement_name, selector)
                                    })?;

                                Ok(Some(match self.next_site_after_relationship(&relationship)? {
                                    Some(site) => site,
                                    None => relationship.into(),
                                }))
                            }

                            _ => Err(format!(
                                "invalid TOSCA path: requirement name not a string: {}",
                                requirement_name.type_name()
                            )),
                        }
                    }

                    // CAPABILITY, <capability_name>, RELATIONSHIP, <idx>, <rel_context> |
                    // CAPABILITY, <capability_name>
                    "CAPABILITY" => {
                        self.iterator.next();
                        let capability_name =
                            self.iterator.next().ok_or_else(|| "invalid TOSCA path: missing capability name")?;

                        match capability_name {
                            Expression::Text(capability_name) => {
                                let capability = path_site
                                    .tosca_capability(capability_name)?
                                    .ok_or_else(|| format!("TOSCA path: capability not found: {}", capability_name))?;

                                Ok(Some(match self.next_site_after_capability(&capability)? {
                                    Some(site) => site,
                                    None => capability.into(),
                                }))
                            }

                            _ => Err(format!(
                                "invalid TOSCA path: capability name not a string: {}",
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
                        let relationship =
                            path_site.tosca_incoming_relationship(requirement_name, selector.clone())?.ok_or_else(
                                || format!("TOSCA path: relationship not found: {} {}", requirement_name, selector),
                            )?;

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

    /// Next selector for node or relationship.
    pub fn next_selector(&mut self) -> Result<ToscaSelector, String> {
        Ok(match self.iterator.peek() {
            Some(index) => match index {
                // ALL
                Expression::Text(text) => match text.as_str() {
                    "ALL" => {
                        self.iterator.next();
                        ToscaSelector::All
                    }

                    _ => Default::default(),
                },

                // <integer_index>
                Expression::Integer(integer) => {
                    self.iterator.next();
                    ToscaSelector::Index(*integer as usize)
                }

                // <integer_index>
                Expression::UnsignedInteger(unsigned_integer) => {
                    self.iterator.next();
                    ToscaSelector::Index(*unsigned_integer as usize)
                }

                _ => Default::default(),
            },

            None => Default::default(),
        })
    }

    /// Next property.
    pub fn next_property<'site>(
        &mut self,
        path_site: &'site Entity,
        read_only: bool,
    ) -> Result<&'site Property, String> {
        let argument = self
            .iterator
            .next()
            .ok_or_else(|| format!("invalid TOSCA path: missing |meta|{}| name", property_or_attribute(read_only)))?;

        match argument {
            Expression::Text(property_name) => {
                let property = path_site.property(property_name).ok_or_else(|| {
                    format!(
                        "TOSCA path: |meta|{}| |name|{}| not found",
                        property_or_attribute(read_only),
                        escape_depiction_markup(argument)
                    )
                })?;

                if property.read_only == read_only {
                    Ok(property)
                } else {
                    Err(format!(
                        "TOSCA path: |meta|{}| |name|{}| found but {}read only",
                        property_or_attribute(read_only),
                        escape_depiction_markup(argument),
                        if read_only { "" } else { "not " },
                    ))
                }
            }

            _ => Err(format!(
                "invalid TOSCA path: |meta|{}| name not a |name|string|: |error|{}|",
                property_or_attribute(read_only),
                argument.type_name()
            )),
        }
    }

    /// Next expression.
    pub fn next_expression(&mut self, property: &Property) -> Result<Expression, String> {
        let Some(property_value) = property.value() else {
            return Err(format!("TOSCA path: no value for property"));
        };

        let mut current_value = &property_value;

        while let Some(argument) = self.iterator.next() {
            let value = current_value.get(argument).ok_or_else(|| {
                format!("TOSCA path: |error|{}| not found in value", escape_depiction_markup(argument))
            })?;

            current_value = value;
        }

        Ok(current_value.clone())
    }
}

fn property_or_attribute(read_only: bool) -> &'static str {
    match read_only {
        true => "attribute",
        false => "property",
    }
}
