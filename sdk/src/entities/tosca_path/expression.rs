use super::super::super::entities::*;

use {
    floria_plugin_sdk::{data::*, entities::*, utils::*},
    std::iter::*,
};

impl<'context> ToscaPathParser<'context> {
    /// Next expression.
    pub fn next_expression(&mut self, property: &Property) -> Result<Expression, String> {
        let Some(property_value) = property.value() else {
            return Err("TOSCA path: no value for |meta|property|".into());
        };

        let mut current_value = &property_value;

        while let Some(argument) = self.iterator.next() {
            let value = current_value.get(argument).ok_or_else(|| {
                format!("TOSCA path: |meta|property| missing: |error|{}|", escape_depiction_markup(argument))
            })?;

            current_value = value;
        }

        Ok(current_value.clone())
    }
}
