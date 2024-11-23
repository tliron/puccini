use super::{
    super::{expression::*, kind::*},
    coerce::*,
    schema::*,
    value::*,
};

use {floria_plugin_sdk::data::*, std::collections::*};

//
// MapSchema
//

/// Map schema.
#[derive(Clone, Debug, Default)]
pub struct MapSchema {
    /// Key.
    pub key: Option<SchemaReference>,

    /// Entry.
    pub entry: Option<SchemaReference>,

    /// Default.
    pub default: Option<Expression>,

    /// Validation.
    pub validation: Option<Expression>,
}

impl MapSchema {
    /// Constructor.
    pub fn new(
        key: Option<SchemaReference>,
        entry: Option<SchemaReference>,
        default: Option<Expression>,
        validation: Option<Expression>,
    ) -> Self {
        Self { key, entry, default, validation }
    }
}

impl Coerce for MapSchema {
    fn default(&self) -> Option<&Expression> {
        self.default.as_ref()
    }

    fn validation(&self) -> Option<&Expression> {
        self.validation.as_ref()
    }

    fn coerce(&self, expression: Expression, schema: &Schema, call_site: &CallSite) -> Result<Expression, String> {
        let expression = expression.must_dispatch_if_call(call_site)?;

        let map = expression.cast_map("map")?;
        let map = map.map();

        let expression = match self.entry {
            Some(reference) => {
                let entry_schema = schema.dereference(reference)?;

                // Key schema is optional (will default to "string" data kind)
                let key_schema = match self.key {
                    Some(reference) => Some(schema.dereference(reference)?),
                    None => None,
                };

                let mut coerced_map = BTreeMap::default();

                for (key, value) in &map.inner {
                    let key = key.clone().must_dispatch_if_call(call_site)?;

                    let key = match key_schema {
                        Some(key_schema) => key_schema.coerce(key, schema, call_site)?,
                        None => key.must_coerce(STRING_DATA_KIND)?,
                    };

                    let value = entry_schema.coerce(value.clone(), schema, call_site)?;

                    coerced_map.insert(key, value);
                }

                coerced_map.into()
            }

            None => expression,
        };

        self.validate(&expression, call_site)?;
        Ok(expression)
    }
}

impl TryFrom<Expression> for MapSchema {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        let map = expression.cast_map("map schema")?;
        let map = map.map();

        let key = get_unsigned_integer_option(map, "key")?;
        let entry = get_unsigned_integer_option(map, "entry")?;
        let default = map.into_get("default").cloned();
        let validation = map.into_get("validation").cloned();

        Ok(Self::new(key, entry, default, validation))
    }
}

// Utils

fn get_unsigned_integer_option(map: &Map, name: &'static str) -> Result<Option<u64>, String> {
    Ok(match map.into_get(name) {
        Some(value) => Some(value.cast_u64(&format!("map schema |meta|{}| key", name))?),
        None => None,
    })
}
