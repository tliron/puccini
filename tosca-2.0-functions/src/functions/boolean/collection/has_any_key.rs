use floria_plugin_sdk::{data::*, utils::*};

/// The $has_any_key function takes two arguments. The first argument is a map. The second argument
/// is a list with the entry_schema matching the key_schema of the first argument. It evaluates to
/// true if there is an entry in the second argument which is equal to a key in the first argument.
pub fn has_any_key(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let map = arguments.next().unwrap().must_evaluate(&call_site)?;
    let map = &map.cast_map("first argument")?.map().inner;

    let keys = arguments.next().unwrap().must_evaluate(&call_site)?;
    let keys = &keys.cast_list("second argument")?.list().inner;

    for key in keys {
        // TODO: check key type?

        if map.contains_key(key) {
            return Ok(Some(true.into()));
        }
    }

    Ok(Some(false.into()))
}
