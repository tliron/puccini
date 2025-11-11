use super::super::super::entities::*;

use {
    floria_plugin_sdk::{data::*, entities::*, utils::*},
    std::iter::*,
};

impl<'own> ToscaPathParser<'own> {
    /// Next input.
    pub fn next_input<'site>(&mut self, path_site: &'site Entity) -> Result<&'site Property, String> {
        let argument = self.iterator.next().ok_or_else(|| "TOSCA path: missing |meta|input| name")?;

        match argument {
            Expression::Text(input_name) => {
                let input_name = String::from("input:") + input_name;
                let input = path_site.property(&input_name).ok_or_else(|| {
                    format!("TOSCA path: |meta|input| not found: {}", escape_depiction_markup(argument))
                })?;

                Ok(input)
            }

            _ => Err(format!("TOSCA path: |meta|input| name not |name|string|: |error|{}|", argument.type_name())),
        }
    }
}
