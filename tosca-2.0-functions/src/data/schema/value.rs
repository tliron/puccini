use super::{coerce::*, list::*, map::*, primitive::*, scalar::*, schema::*, r#struct::*};

use floria_plugin_sdk::{data::*, errors, *};

/// Schema reference.
pub type SchemaReference = u64;

//
// ValueSchema
//

/// Value schema.
#[derive(Clone, Debug)]
pub enum ValueSchema {
    /// Reference.
    Reference(SchemaReference),

    /// Primitive schema.
    Primitive(PrimitiveSchema),

    /// Scalar schema.
    Scalar(ScalarSchema),

    /// List schema.
    List(ListSchema),

    /// Map schema.
    Map(MapSchema),

    /// Struct schema.
    Struct(StructSchema),
}

impl ValueSchema {
    /// Coerce into the schema.
    pub fn coerce(
        &self,
        expression: Expression,
        schema: &Schema,
        call_site: &CallSite,
    ) -> Result<Expression, DispatchError> {
        match self {
            Self::Reference(reference) => schema.dereference(*reference)?.coerce(expression, schema, call_site),
            Self::Primitive(primitive) => primitive.coerce(expression, schema, call_site),
            Self::Scalar(scalar) => scalar.coerce(expression, schema, call_site),
            Self::List(list) => list.coerce(expression, schema, call_site),
            Self::Map(map) => map.coerce(expression, schema, call_site),
            Self::Struct(struct_) => struct_.coerce(expression, schema, call_site),
        }
    }

    /// Coerce into the schema.
    pub fn coerce_option(
        &self,
        expression: Option<Expression>,
        schema: &Schema,
        call_site: &CallSite,
    ) -> Result<Option<Expression>, DispatchError> {
        match self {
            Self::Reference(reference) => schema.dereference(*reference)?.coerce_option(expression, schema, call_site),
            Self::Primitive(primitive) => primitive.coerce_option(expression, schema, call_site),
            Self::Scalar(scalar) => scalar.coerce_option(expression, schema, call_site),
            Self::List(list) => list.coerce_option(expression, schema, call_site),
            Self::Map(map) => map.coerce_option(expression, schema, call_site),
            Self::Struct(struct_) => struct_.coerce_option(expression, schema, call_site),
        }
    }
}

impl TryFrom<Expression> for ValueSchema {
    type Error = DispatchError;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        if let Expression::Map(map_resource) = &expression {
            let map = map_resource.map();

            let Some(kind) = map.into_get("kind") else {
                return Err("value schema missing |meta|kind| key".into());
            };

            let Expression::Text(kind) = kind else {
                return Err(format!("value schema |meta|kind| key not a |name|string|: |error|{}|", kind.type_name()));
            };

            match kind.as_str() {
                "scalar" => {
                    let scalar = ScalarSchema::try_from(expression)?;
                    return Ok(Self::Scalar(scalar));
                }

                "list" => {
                    let list = ListSchema::try_from(expression)?;
                    return Ok(Self::List(list));
                }

                "map" => {
                    let map = MapSchema::try_from(expression)?;
                    return Ok(Self::Map(map));
                }

                "struct" => {
                    let r#struct = StructSchema::try_from(expression)?;
                    return Ok(Self::Struct(r#struct));
                }

                _ => {
                    let primitive: PrimitiveSchema = expression.try_into()?;
                    return Ok(Self::Primitive(primitive));
                }
            }
        }

        match expression {
            Expression::UnsignedInteger(unsigned_integer) => Ok(Self::Reference(unsigned_integer)),
            Expression::Text(text) => Ok(Self::Primitive(PrimitiveSchema::new(text, None, None))),
            _ => Err(errors::not_of_types_for("value schema", &expression, &["map", "string", "unsigned integer"])),
        }
    }
}
