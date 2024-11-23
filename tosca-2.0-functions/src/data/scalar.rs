use super::{comparator::*, number::*, scalar_schema::*};

use {
    floria_plugin_sdk::data::*,
    std::{collections::*, fmt, str::*},
};

const NOTATION_ERROR: &str = "scalar is not \"<number> <unit>\"";

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
}

impl Scalar {
    /// Constructor.
    pub fn new(number: Number, unit: String) -> Self {
        Self { number, unit }
    }

    /// Constructor.
    pub fn new_from_any(any: &Any, schema: &ScalarSchema) -> Result<Self, String> {
        match any {
            Any::Text(text) => Self::new_from_str(text, schema),
            Any::AnyMap(any_map) => Self::new_from_map(&any_map.to_map().inner, schema),
            _ => Err("scalar is not a string or a map".into()),
        }
    }

    /// Constructor.
    pub fn new_from_str(representation: &str, schema: &ScalarSchema) -> Result<Self, String> {
        let mut split = representation.split_whitespace();

        let Some(number) = split.next() else {
            return Err(NOTATION_ERROR.into());
        };

        let Some(unit) = split.next() else {
            return Err(NOTATION_ERROR.into());
        };

        if split.next().is_some() {
            return Err(NOTATION_ERROR.into());
        }

        let mut number = Number::from_str(number)?;

        let unit_factor = schema.unit_factor(unit)?;
        let unit = schema.canonical_unit()?;

        if !unit_factor.is_one() {
            number = number.multiply(unit_factor)?;
        }

        if schema.is_integer() {
            Ok(Self::new(Number::Integer(number.try_into()?), unit))
        } else {
            Ok(Self::new(Number::Float(number.try_into()?), unit))
        }
    }

    /// Constructor.
    pub fn new_from_map(map: &BTreeMap<Any, Any>, schema: &ScalarSchema) -> Result<Self, String> {
        let mut number = None;
        let mut unit = None;

        for (key, value) in map {
            match key {
                Any::Text(text) => match text.as_str() {
                    "number" => {
                        number = Some(value.try_into().map_err(|_error| "scalar \"number\" key is not an integer")?);
                    }

                    "unit" => {
                        unit = Some(match value {
                            Any::Text(text) => {
                                let _ = schema.unit_factor(text)?;
                                text.clone()
                            }

                            _ => return Err("scalar \"unit\" key is not a string".into()),
                        });
                    }

                    _ => return Err(format!("scalar has unsupported key: {}", key)),
                },

                _ => return Err(format!("scalar has unsupported key: {}", key)),
            }
        }

        if let Some(number) = number
            && let Some(unit) = unit
        {
            Ok(Self::new(number, unit))
        } else {
            Err("scalar is missing keys".into())
        }
    }
}

impl Comparator for Scalar {
    fn comparator(&self) -> Any {
        self.number.into()
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.number, self.unit)
    }
}

impl Into<Any> for Scalar {
    fn into(self) -> Any {
        let number: Any = self.number.into();
        normal_map!(("number", number), ("unit", self.unit))
    }
}
