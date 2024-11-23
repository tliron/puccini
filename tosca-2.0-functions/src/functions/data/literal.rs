use super::super::super::data::*;

use floria_plugin_sdk::data::*;

/// Construct a literal.
pub fn literal(arguments: Vec<Any>, site: Site) -> Result<Any, String> {
    if arguments.len() != 1 {
        return Err("must have one argument".into());
    }

    let value = arguments.first().expect("first argument");
    let value = resolve(&site, value, false)?;

    return Ok(value);
}
