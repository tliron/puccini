use super::{super::call_site::*, schema::*};

use floria_plugin_sdk::data::*;

//
// Coerce
//

/// Coerce.
pub trait Coerce {
    /// Default.
    fn default(&self) -> Option<&Expression>;

    /// Validation.
    fn validation(&self) -> Option<&Expression>;

    /// Coerce into the schema.
    fn coerce(&self, expression: Expression, schema: &Schema, call_site: &CallSite) -> Result<Expression, String>;

    /// Coerce into the schema.
    fn coerce_option(
        &self,
        mut expression: Option<Expression>,
        schema: &Schema,
        call_site: &CallSite,
    ) -> Result<Option<Expression>, String> {
        if expression.is_none()
            && let Some(default) = self.default()
        {
            expression = Some(default.clone());
        }

        Ok(match expression {
            Some(expression) => Some(self.coerce(expression, schema, call_site)?),
            None => expression,
        })
    }

    /// Validate the expression.
    fn validate(&self, expression: &Expression, call_site: &CallSite) -> Result<(), String> {
        if let Some(Expression::Call(call_resource)) = self.validation() {
            push_call_site_value(expression.clone())?;
            call_resource.call().dispatch(call_site)?;
            pop_call_site_value()?;
        }

        Ok(())
    }
}
