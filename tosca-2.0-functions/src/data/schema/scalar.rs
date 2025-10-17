use super::{
    super::{kind::*, scalar::*},
    coerce::*,
    schema::*,
};

use {
    floria_plugin_sdk::{data::*, utils::*, *},
    std::collections::*,
};

//
// ScalarSchema
//

/// Scalar schema.
#[derive(Clone, Debug, Default)]
pub struct ScalarSchema {
    /// Data kind.
    pub data_kind: Option<String>,

    /// Units and their factors.
    pub units: BTreeMap<String, Number>,

    /// Canonical unit.
    pub canonical_unit: String,

    /// Canonical factor.
    pub canonical_factor: Number,

    /// Prefixes and their factors.
    pub prefixes: BTreeMap<String, Number>,

    /// Default.
    pub default: Option<Expression>,

    /// Validation.
    pub validation: Option<Expression>,
}

impl ScalarSchema {
    /// Constructor.
    pub fn new(
        data_kind: Option<String>,
        units: BTreeMap<String, Number>,
        canonical_unit: String,
        canonical_factor: Number,
        prefixes: BTreeMap<String, Number>,
        default: Option<Expression>,
        validation: Option<Expression>,
    ) -> Self {
        Self { data_kind, units, canonical_unit, canonical_factor, prefixes, default, validation }
    }

    /// Unit factor.
    pub fn unit_factor(&self, unit: &str) -> Result<Number, DispatchError> {
        find_unit_factor(unit, &self.units, &self.prefixes)
    }

    /// True if data kind is integer. Defaults to false.
    pub fn is_integer(&self) -> bool {
        self.data_kind.as_ref().map_or(false, |data_kind| data_kind == INTEGER_DATA_KIND)
    }
}

impl Coerce for ScalarSchema {
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
        let expression = Scalar::new_from_expression(expression, self)?.into();
        self.validate(&expression, call_site)?;
        Ok(expression)
    }
}

impl TryFrom<Expression> for ScalarSchema {
    type Error = DispatchError;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        let map = expression.cast_map("scalar schema")?;
        let map = map.map();

        let data_kind = get_string_option(map, "data_kind")?;
        let units = get_map(map, "units")?;
        let canonical_unit = get_string_option(map, "canonical_unit")?;
        let prefixes = get_map(map, "prefixes")?;
        let default = map.into_get("default").cloned();
        let validation = map.into_get("validation").cloned();

        let (canonical_unit, canonical_factor) = find_canonical_unit_and_factor(canonical_unit, &units, &prefixes)?;
        Ok(Self::new(data_kind, units, canonical_unit, canonical_factor, prefixes, default, validation))
    }
}

impl Into<Expression> for ScalarSchema {
    fn into(self) -> Expression {
        let mut map = BTreeMap::default();

        if let Some(data_kind) = self.data_kind {
            map.insert("data_kind".into(), data_kind.into());
        }

        if !self.units.is_empty() {
            let units: BTreeMap<_, _> = self.units.into_iter().map(|(key, value)| (key.into(), value.into())).collect();
            map.insert("units".into(), units.into());
        }

        map.insert("canonical_unit".into(), self.canonical_unit.into());

        if !self.prefixes.is_empty() {
            let prefixes: BTreeMap<_, _> =
                self.prefixes.into_iter().map(|(key, value)| (key.into(), value.into())).collect();
            map.insert("prefixes".into(), prefixes.into());
        }

        map.into()
    }
}

fn get_string_option(map: &Map, name: &'static str) -> Result<Option<String>, DispatchError> {
    Ok(match map.into_get(name) {
        Some(value) => Some(value.cast_string_clone(&format!("scalar schema |meta|{}| key", name))?),
        None => None,
    })
}

fn get_map(map: &Map, name: &'static str) -> Result<BTreeMap<String, Number>, DispatchError> {
    match map.into_get(name) {
        Some(value) => {
            let map = value.cast_map(&format!("scalar schema |meta|{}| key", name))?;
            let mut result = BTreeMap::default();

            for (key, value) in &map.map().inner {
                let key = key.cast_string_clone(&format!("scalar schema |meta|{}| key |name|map| key", name))?;

                let value = value
                    .try_into()
                    .map_err(|error| format!("scalar schema |meta|{}| key |name|map| value: {}", name, error))?;

                result.insert(key.clone(), value);
            }

            Ok(result)
        }

        None => Ok(Default::default()),
    }
}

fn find_unit_factor(
    unit: &str,
    units: &BTreeMap<String, Number>,
    prefixes: &BTreeMap<String, Number>,
) -> Result<Number, DispatchError> {
    if prefixes.is_empty() {
        for (unit_, unit_factor) in units {
            if unit == unit_ {
                return Ok(*unit_factor);
            }
        }
    } else {
        for (prefix, prefix_factor) in prefixes {
            for (unit_, unit_factor) in units {
                let unit_ = prefix.clone() + unit_;
                if unit == unit_ {
                    let factor = prefix_factor.mul(*unit_factor, false)?;
                    return Ok(factor.into());
                }
            }
        }
    }

    Err(format!("unsupported scalar unit: |error|{}|", escape_depiction_markup(unit)))
}

fn find_canonical_unit_and_factor(
    canonical_unit: Option<String>,
    units: &BTreeMap<String, Number>,
    prefixes: &BTreeMap<String, Number>,
) -> Result<(String, Number), DispatchError> {
    match canonical_unit {
        Some(canonical_unit) => {
            let factor = find_unit_factor(&canonical_unit, units, prefixes)?;
            Ok((canonical_unit, factor))
        }

        None => {
            let mut canonical = None;
            for (unit, factor) in units {
                if factor.is_one() {
                    if canonical.is_some() {
                        return Err("multiple candidates for canonical scalar unit".into());
                    }

                    canonical = Some((unit, factor));
                }
            }

            match canonical {
                Some((unit, factor)) => Ok((unit.clone(), *factor)),
                None => Err("no canonical scalar unit".into()),
            }
        }
    }
}
