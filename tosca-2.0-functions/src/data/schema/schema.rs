use super::value::*;

use floria_plugin_sdk::{data::*, *};

//
// Schema
//

/// Schema.
#[derive(Clone, Debug, Default)]
pub struct Schema {
    /// Values.
    pub values: Vec<ValueSchema>,
}

impl Schema {
    /// Root schema.
    pub fn root(&self) -> Result<&ValueSchema, DispatchError> {
        self.dereference(0)
    }

    /// Dereference.
    pub fn dereference(&self, reference: SchemaReference) -> Result<&ValueSchema, DispatchError> {
        match self.values.get(reference as usize) {
            Some(schema) => match schema {
                ValueSchema::Reference(reference) => self.dereference(*reference),
                _ => Ok(schema),
            },

            None => Err(format!("value schema not found: |error|{}|", reference)),
        }
    }

    /// Coerce into the schema.
    pub fn coerce(&self, expression: Expression, call_site: &CallSite) -> Result<Expression, DispatchError> {
        self.root()?.coerce(expression, self, call_site)
    }
}

impl TryFrom<Expression> for Schema {
    type Error = DispatchError;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::List(list_resource) => {
                let list = list_resource.list();

                let mut schemas = Vec::with_capacity(list.inner.len());
                for item in &list.inner {
                    schemas.push(ValueSchema::try_from(item.clone())?);
                }

                Ok(Schema { values: schemas })
            }

            _ => Ok(Schema { values: vec![ValueSchema::try_from(expression)?] }),
        }
    }
}
