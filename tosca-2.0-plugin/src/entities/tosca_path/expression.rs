use super::super::super::entities::*;

use {
    floria_plugin_sdk::{data::*, entities::*, utils::*},
    std::iter::*,
};

impl<'own> ToscaPathParser<'own> {
    /// Next expression.
    pub fn next_expression(&mut self, property: &Property) -> Result<Expression, String> {
        let Some(property_value) = property.value() else {
            return Err(format!("TOSCA path: no value for |meta|{}|", property_or_attribute(property)));
        };

        let mut current_value = &property_value;

        while let Some(argument) = self.iterator.next() {
            let value = current_value.get(argument).ok_or_else(|| {
                format!(
                    "TOSCA path: |meta|{}| missing: |error|{}|",
                    property_or_attribute(property),
                    escape_depiction_markup(argument)
                )
            })?;

            current_value = value;
        }

        Ok(current_value.clone())
    }
}

fn property_or_attribute(property: &Property) -> &'static str {
    match property.read_only {
        true => "attribute",
        false => "property",
    }
}
