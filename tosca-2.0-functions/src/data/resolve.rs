use super::{comparator::*, scalar::*, scalar_schema::*, timestamp::*, version::*};

use floria_plugin_sdk::data::*;

/// Resolve.
pub fn resolve(site: &Site, value: &Any, comparator: bool) -> Result<Any, String> {
    if let Some(data_kind) = site.get_property_metadata_string("tosca:data-kind")? {
        match data_kind.as_str() {
            "Scalar" => {
                let data_kind = site.get_property_metadata_string("tosca:scalar-data-kind")?;
                let units = site.get_property_metadata_map("tosca:scalar-units")?.unwrap_or_default();
                let canonical_unit = site.get_property_metadata_string("tosca:scalar-canonical-unit")?;
                let prefixes = site.get_property_metadata_map("tosca:scalar-prefixes")?.unwrap_or_default();
                let schema = ScalarSchema::new(&data_kind, &units, &canonical_unit, &prefixes);

                let scalar = Scalar::new_from_any(value, &schema)?;
                return Ok(if comparator { scalar.comparator() } else { scalar.into() });
            }

            "Timestamp" => {
                let timestamp: Timestamp = value.try_into()?;
                return Ok(if comparator { timestamp.comparator() } else { timestamp.into() });
            }

            "Version" => {
                let version: Version = value.try_into()?;
                return Ok(if comparator { version.comparator() } else { version.into() });
            }

            _ => {}
        }
    }

    Ok(value.clone())
}
