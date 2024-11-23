use super::{coerce::*, schema::*, value::*};

use {
    floria_plugin_sdk::{data::*, utils::*},
    std::collections::*,
};

//
// StructSchema
//

/// Struct schema.
#[derive(Clone, Debug, Default)]
pub struct StructSchema {
    /// Fields.
    pub fields: BTreeMap<String, StructSchemaField>,

    /// Default.
    pub default: Option<Expression>,

    /// Validation.
    pub validation: Option<Expression>,
}

impl StructSchema {
    /// Constructor.
    pub fn new(
        fields: BTreeMap<String, StructSchemaField>,
        default: Option<Expression>,
        validation: Option<Expression>,
    ) -> Self {
        Self { fields, default, validation }
    }
}

impl Coerce for StructSchema {
    fn default(&self) -> Option<&Expression> {
        self.default.as_ref()
    }

    fn validation(&self) -> Option<&Expression> {
        self.validation.as_ref()
    }

    fn coerce(&self, expression: Expression, schema: &Schema, call_site: &CallSite) -> Result<Expression, String> {
        let expression = expression.must_dispatch_if_call(call_site)?;

        let expression = match expression {
            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let mut coerced_map = BTreeMap::default();

                // Do we have unsupported fields?

                for field_name in map.inner.keys() {
                    let Expression::Text(field_name) = field_name else {
                        return Err(format!(
                            "struct field name not a |name|string|: |error|{}|",
                            field_name.type_name()
                        ));
                    };

                    if !self.fields.contains_key(field_name) {
                        return Err(format!("not a struct field: |error|{:?}|", escape_depiction_markup(field_name)));
                    }
                }

                // Coerce fields

                for (field_name, field) in &self.fields {
                    let field_name_expression = field_name.clone().into();
                    let value = map.inner.get(&field_name_expression).cloned();
                    let value = field.value_schema.coerce_option(value, schema, call_site)?;

                    match value {
                        Some(value) => {
                            coerced_map.insert(field_name_expression, value);
                        }

                        None => {
                            if field.required {
                                return Err(format!(
                                    "missing required struct field: |error|{}|",
                                    escape_depiction_markup(field_name)
                                ));
                            }
                        }
                    }
                }

                coerced_map.into()
            }

            _ => return Err(format!("struct not a |name|map|: |error|{}|", expression.type_name())),
        };

        self.validate(&expression, call_site)?;
        Ok(expression)
    }
}

impl TryFrom<Expression> for StructSchema {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let fields = get_map(map, "fields")?;
                let default = map.into_get("default").cloned();
                let validation = map.into_get("validation").cloned();

                Ok(Self::new(fields, default, validation))
            }

            _ => Err(format!("struct schema not a |name|map|: |error|{}|", expression.type_name())),
        }
    }
}

fn get_map(map: &Map, name: &'static str) -> Result<BTreeMap<String, StructSchemaField>, String> {
    match map.into_get(name) {
        Some(Expression::Map(map_resource)) => {
            let mut map = BTreeMap::default();

            for (key, value) in &map_resource.map().inner {
                let Expression::Text(key) = key else {
                    return Err(format!(
                        "struct schema |meta|{}| key |name|map| key not a |name|string|: |error|{}|",
                        name,
                        key.type_name()
                    ));
                };

                let field = match value {
                    // List = [schema, required]
                    Expression::List(list_resource) => {
                        let list = list_resource.list();

                        let length = list.inner.len();
                        if length != 2 {
                            return Err(format!(
                                "struct schema |meta|{}| key value list length not 2: |error|{}|",
                                name, length
                            ));
                        }

                        let value_schema = list.inner.get(0).expect("first item");
                        let value_schema = value_schema
                            .clone()
                            .try_into()
                            .map_err(|error| format!("struct schema |meta|{}| key first item: {}", name, error))?;

                        let required = list.inner.get(1).expect("second item");
                        let Expression::Boolean(required) = required else {
                            return Err(format!(
                                "struct schema |meta|{}| key second item not a |name|boolean|: |error|{}|",
                                name,
                                required.type_name()
                            ));
                        };

                        StructSchemaField::new(value_schema, *required)
                    }

                    // Just the schema
                    // ("required" defaults to true)
                    _ => {
                        let value_schema: ValueSchema = value
                            .clone()
                            .try_into()
                            .map_err(|error| format!("struct schema |meta|{}| key: {}", name, error))?;

                        value_schema.into()
                    }
                };

                map.insert(key.clone(), field);
            }

            Ok(map)
        }

        Some(value) => {
            return Err(format!("struct schema |meta|{}| key not a |name|map|: |error|{}|", name, value.type_name()));
        }

        None => Ok(Default::default()),
    }
}

//
// StructSchemaField
//

/// Struct schema field.
#[derive(Clone, Debug)]
pub struct StructSchemaField {
    /// Value schema.
    pub value_schema: ValueSchema,

    /// Required.
    pub required: bool,
}

impl StructSchemaField {
    /// Constructor.
    pub fn new(value_schema: ValueSchema, required: bool) -> Self {
        Self { value_schema, required }
    }
}

impl From<ValueSchema> for StructSchemaField {
    fn from(value_schema: ValueSchema) -> Self {
        Self::new(value_schema, true)
    }
}
