use super::{super::expression::*, coerce::*, schema::*};

use floria_plugin_sdk::{data::*, errors, *};

//
// PrimitiveSchema
//

/// Primitive schema.
#[derive(Clone, Debug, Default)]
pub struct PrimitiveSchema {
    /// Data kind.
    pub data_kind: String,

    /// Default.
    pub default: Option<Expression>,

    /// Validation.
    pub validation: Option<Expression>,
}

impl PrimitiveSchema {
    /// Constructor.
    pub fn new(data_kind: String, default: Option<Expression>, validation: Option<Expression>) -> Self {
        Self { data_kind, default, validation }
    }
}

impl Coerce for PrimitiveSchema {
    fn default(&self) -> Option<&Expression> {
        self.default.as_ref()
    }

    fn validation(&self) -> Option<&Expression> {
        self.validation.as_ref()
    }

    fn coerce(
        &self,
        expression: Expression,
        _schema: &Schema,
        call_site: &CallSite,
    ) -> Result<Expression, DispatchError> {
        let expression = expression.must_dispatch_if_call(call_site)?;
        let expression = expression.must_coerce(&self.data_kind)?;
        self.validate(&expression, call_site)?;
        Ok(expression)
    }
}

impl TryFrom<Expression> for PrimitiveSchema {
    type Error = DispatchError;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Map(map_resource) => {
                let map = map_resource.map();

                let Some(kind) = map.into_get("kind") else {
                    return Err("primitive schema missing |meta|kind| key".into());
                };

                let Expression::Text(kind) = kind else {
                    return Err(errors::not_of_types_for("primitive schema |meta|kind| key", kind, &["string"]));
                };

                let default = map.into_get("default").cloned();
                let validation = map.into_get("validation").cloned();

                Ok(Self::new(kind.clone(), default, validation))
            }

            Expression::Text(text) => Ok(Self::new(text, None, None)),

            _ => Err(errors::not_of_types_for("primitive schema", &expression, &["map", "string"])),
        }
    }
}
