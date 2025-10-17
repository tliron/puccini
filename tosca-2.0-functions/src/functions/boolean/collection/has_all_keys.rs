use floria_plugin_sdk::{data::*, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $has_all_keys function takes two arguments. The first argument is a map. The second
/// argument is a list with the entry_schema matching the key_schema of the first argument. It
/// evaluates to true if for all entries in the second argument there is an equal value key in the
/// first argument.
pub fn has_all_keys(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let map = arguments.next().unwrap().must_evaluate(&call_site)?;
    let map = &map.cast_map("first argument")?.map().inner;

    let keys = arguments.next().unwrap().must_evaluate(&call_site)?;
    let keys = &keys.cast_list("second argument")?.list().inner;

    for key in keys {
        // TODO: check key type?

        if !map.contains_key(key) {
            return Ok(Some(false.into()));
        }
    }

    Ok(Some(true.into()))
}
