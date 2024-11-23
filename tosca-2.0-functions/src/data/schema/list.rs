use super::{coerce::*, schema::*, value::*};

use floria_plugin_sdk::data::*;

//
// ListSchema
//

/// List schema.
#[derive(Clone, Debug, Default)]
pub struct ListSchema {
    /// Entry.
    pub entry: Option<SchemaReference>,

    /// Default.
    pub default: Option<Expression>,

    /// Validation.
    pub validation: Option<Expression>,
}

impl ListSchema {
    /// Constructor.
    pub fn new(entry: Option<SchemaReference>, default: Option<Expression>, validation: Option<Expression>) -> Self {
        Self { entry, default, validation }
    }
}

impl Coerce for ListSchema {
    fn default(&self) -> Option<&Expression> {
        self.default.as_ref()
    }

    fn validation(&self) -> Option<&Expression> {
        self.validation.as_ref()
    }

    fn coerce(&self, expression: Expression, schema: &Schema, call_site: &CallSite) -> Result<Expression, String> {
        let expression = expression.must_dispatch_if_call(call_site)?;

        let Expression::List(list_resource) = &expression else {
            return Err(format!("not a |name|list|: |error|{}|", expression.type_name()));
        };
        let list = list_resource.list();

        let expression = match self.entry {
            Some(reference) => {
                let entry_schema = schema.get(reference)?;

                let mut coerced_list = Vec::with_capacity(list.inner.len());
                for item in &list.inner {
                    coerced_list.push(entry_schema.coerce(item.clone(), schema, call_site)?);
                }

                coerced_list.into()
            }

            None => expression,
        };

        self.validate(&expression, call_site)?;
        Ok(expression)
    }
}

impl TryFrom<Expression> for ListSchema {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let entry = get_unsigned_integer_option(map, "entry")?;
                let default = map.into_get("default").cloned();
                let validation = map.into_get("validation").cloned();

                Ok(Self::new(entry, default, validation))
            }

            _ => Err(format!("list schema not a |name|map|: |error|{}|", expression.type_name())),
        }
    }
}

fn get_unsigned_integer_option(map: &Map, name: &'static str) -> Result<Option<u64>, String> {
    match map.into_get(name) {
        Some(Expression::UnsignedInteger(unsigned_integer)) => Ok(Some(*unsigned_integer)),
        Some(value) => Err(format!(
            "list schema |meta|{}| key not an |name|unsigned integer|: |error|{}|",
            name,
            value.type_name()
        )),
        None => Ok(None),
    }
}
