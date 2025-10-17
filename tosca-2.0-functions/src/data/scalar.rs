use super::{comparator::*, schema::*};

use {
    floria_plugin_sdk::{data::*, errors},
    std::{collections::*, fmt, str::*},
};

/// Scalar custom kind.
pub const SCALAR_CUSTOM_KIND: &str = "tosca_2_0:scalar";

const NOTATION_ERROR: &str = "scalar not \"<number> <unit>\"";

//
// Scalar
//

/// (Quoted from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The TOSCA scalar types can be used to define scalar values along with an associated unit.
#[derive(Clone, Debug, Default)]
pub struct Scalar {
    /// Number.
    pub number: Number,

    /// Unit.
    pub unit: String,

    /// Schema.
    pub schema: ScalarSchema,
}

impl Scalar {
    /// Constructor.
    pub fn new(number: Number, unit: String, schema: ScalarSchema) -> Self {
        Self { number, unit, schema }
    }

    /// Constructor.
    pub fn new_canonical(mut number: Number, unit: &str, schema: ScalarSchema) -> Result<Self, String> {
        let factor = schema.unit_factor(unit)?;

        if !factor.is_one() {
            number = number.mul(factor, false)?;
        }

        let number =
            if schema.is_integer() { Number::Integer(number.try_into()?) } else { Number::Float(number.try_into()?) };

        Ok(Self::new(number, schema.canonical_unit.clone(), schema))
    }

    /// Constructor.
    pub fn new_from_expression(expression: Expression, schema: &ScalarSchema) -> Result<Self, String> {
        match expression {
            Expression::Text(text) => Self::new_from_str(&text, schema),
            Expression::Custom(custom_resource) => custom_resource.custom().try_into(),
            _ => Err(errors::not_of_types_for("scalar", &expression, &["string", "custom data type"])),
        }
    }

    /// Constructor.
    pub fn new_from_str(representation: &str, schema: &ScalarSchema) -> Result<Self, String> {
        let mut split = representation.split_whitespace();

        let Some(number) = split.next() else {
            return Err(NOTATION_ERROR.into());
        };
        let number = Number::from_str(number)?;

        let Some(unit) = split.next() else {
            return Err(NOTATION_ERROR.into());
        };

        if split.next().is_some() {
            return Err(NOTATION_ERROR.into());
        }

        Self::new_canonical(number, unit, schema.clone())
    }

    /// True if canonical.
    pub fn is_canonical(&self) -> bool {
        self.unit == self.schema.canonical_unit
    }

    /// Canonical number.
    pub fn canonical(&self) -> Result<Number, String> {
        if self.is_canonical() { Ok(self.number) } else { self.number.mul(self.schema.canonical_factor, false) }
    }
}

impl Comparator for Scalar {
    fn comparator(&self) -> Result<Expression, String> {
        Ok(self.canonical()?.into())
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.number, self.unit)
    }
}

// Conversions

impl TryFrom<Expression> for Scalar {
    type Error = String;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        expression.cast_custom("scalar")?.custom().try_into()
    }
}

impl TryFrom<&Custom> for Scalar {
    type Error = String;

    fn try_from(custom: &Custom) -> Result<Self, Self::Error> {
        custom.assert_kind(SCALAR_CUSTOM_KIND, "scalar")?;

        let map = custom.inner.cast_map("scalar custom data type")?;
        let map = &map.map().inner;

        let Some(schema) = map.get(&"schema".into()) else {
            return Err("scalar missing |meta|schema| key".into());
        };
        let schema: ScalarSchema = schema.clone().try_into()?;

        let Some(number) = map.get(&"number".into()) else {
            return Err("scalar missing |meta|number| key".into());
        };
        let number: Number = number.try_into().map_err(|error| format!("scalar |meta|number| key value: {}", error))?;

        let Some(unit) = map.get(&"unit".into()) else {
            return Err("scalar missing |meta|unit| key".into());
        };
        let unit = unit.cast_string_clone("scalar |meta|unit| key")?;

        Ok(Self::new_canonical(number, &unit, schema)?)
    }
}

impl Into<Expression> for Scalar {
    fn into(self) -> Expression {
        let number: Expression = self.number.into();
        let map = BTreeMap::from([
            ("number".into(), number),
            ("unit".into(), self.unit.into()),
            ("schema".into(), self.schema.into()),
        ]);
        Custom::new(SCALAR_CUSTOM_KIND.into(), map.into()).into()
    }
}
