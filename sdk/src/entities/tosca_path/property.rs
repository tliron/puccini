use super::super::super::entities::*;

use {
    floria_plugin_sdk::{data::*, entities::*, utils::*},
    std::iter::*,
};

impl<'context> ToscaPathParser<'context> {
    /// Next property.
    pub fn next_property<'site>(
        &mut self,
        path_site: &'site Entity,
        read_only: bool,
    ) -> Result<&'site Property, String> {
        let argument = self
            .iterator
            .next()
            .ok_or_else(|| format!("TOSCA path: missing |meta|{}| name", property_or_attribute(read_only)))?;

        match argument {
            Expression::Text(property_name) => {
                let property = path_site.property(property_name).ok_or_else(|| {
                    format!(
                        "TOSCA path: |meta|{}| not found: {}",
                        property_or_attribute(read_only),
                        escape_depiction_markup(argument)
                    )
                })?;

                if property.read_only == read_only {
                    Ok(property)
                } else {
                    Err(format!(
                        "TOSCA path: |meta|{}| found but {}read only: {}",
                        property_or_attribute(read_only),
                        if read_only { "" } else { "not " },
                        escape_depiction_markup(argument),
                    ))
                }
            }

            _ => Err(format!(
                "TOSCA path: |meta|{}| name not |name|string|: |error|{}|",
                property_or_attribute(read_only),
                argument.type_name()
            )),
        }
    }
}

fn property_or_attribute(read_only: bool) -> &'static str {
    match read_only {
        true => "attribute",
        false => "property",
    }
}
