use super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// This function is used as an argument inside validation functions. It returns the value of the
/// property, attribute, or parameter for which the validation clause is defined.
pub fn value(arguments: Vec<Any>, site: Site) -> Result<Any, String> {
    if !arguments.is_empty() {
        return Err("must have no arguments".into());
    }

    if let Some(value) = site.property_value()? {
        let value = resolve(&site, &value, false)?;
        return Ok(value);
    }

    Err("value not available here".into())
}
