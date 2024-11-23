use super::number::*;

use floria_plugin_sdk::data::*;

//
// ScalarSchema
//

/// Scalar schema.
pub struct ScalarSchema<'own> {
    /// Data kind.
    pub data_kind: &'own Option<String>,

    /// Units.
    pub units: &'own Map,

    /// Canonical unit.
    pub canonical_unit: &'own Option<String>,

    /// Prefixes.
    pub prefixes: &'own Map,
}

impl<'own> ScalarSchema<'own> {
    /// Constructor.
    pub fn new(
        data_kind: &'own Option<String>,
        units: &'own Map,
        canonical_unit: &'own Option<String>,
        prefixes: &'own Map,
    ) -> Self {
        Self { data_kind, units, canonical_unit, prefixes }
    }

    /// True if data kind is integer.
    pub fn is_integer(&self) -> bool {
        self.data_kind.as_ref().map_or(false, |data_kind| data_kind == "Integer")
    }

    /// Find canonical unit.
    pub fn canonical_unit(&self) -> Result<String, String> {
        match self.canonical_unit {
            Some(canonical_unit) => {
                let _ = self.unit_factor(canonical_unit)?;
                Ok(canonical_unit.clone())
            }

            None => {
                let mut canonical_unit = None;
                for (unit, factor) in &self.units.inner {
                    if let Any::Text(unit) = unit {
                        if Number::try_from(factor)?.is_one() {
                            if canonical_unit.is_some() {
                                return Err("multiple candidates for canonical unit".into());
                            }

                            canonical_unit = Some(unit);
                        }
                    }
                }

                match canonical_unit {
                    Some(canonical_unit) => Ok(canonical_unit.clone()),
                    None => Err("no canonical unit".into()),
                }
            }
        }
    }

    /// Find factor for unit.
    pub fn unit_factor(&self, unit: &str) -> Result<Number, String> {
        if !self.prefixes.inner.is_empty() {
            for (prefix, prefix_factor) in &self.prefixes.inner {
                if let Any::Text(prefix) = prefix {
                    for (unit_, unit_factor) in &self.units.inner {
                        if let Any::Text(unit_) = unit_ {
                            let unit_ = format!("{}{}", prefix, unit_);
                            if unit == unit_ {
                                let prefix_factor = Number::try_from(prefix_factor)?;
                                let unit_factor = Number::try_from(unit_factor)?;
                                let factor = prefix_factor.multiply(unit_factor)?;
                                return Ok(factor.into());
                            }
                        }
                    }
                }
            }
        } else {
            for (unit_, unit_factor) in &self.units.inner {
                if let Any::Text(unit_) = unit_ {
                    if unit == unit_ {
                        let unit_factor = Number::try_from(unit_factor)?;
                        return Ok(unit_factor.into());
                    }
                }
            }
        }

        Err(format!("unsupported unit: {}", unit))
    }
}
