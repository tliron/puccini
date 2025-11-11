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

            return Ok(match kind.as_str() {
                "scalar" => Self::Scalar(expression.try_into()?),
                "list" => Self::List(expression.try_into()?),
                "map" => Self::Map(expression.try_into()?),
                "struct" => Self::Struct(expression.try_into()?),
                _ => Self::Primitive(expression.try_into()?),
            });
        }

        match expression {
            Expression::UnsignedInteger(unsigned_integer) => Ok(Self::Reference(unsigned_integer)),
            Expression::Text(text) => Ok(Self::Primitive(PrimitiveSchema::new(text, None, None))),
            _ => Err(errors::not_of_types_for("value schema", &expression, &["map", "string", "unsigned integer"])),
        }
    }
}
