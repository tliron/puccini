use floria_plugin_sdk::{data::*, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $has_key function takes two arguments. The first argument is a map. The second argument is
/// of the type matching the key_schema of the first argument. It evaluates to true if the second
/// argument is a key in any of the key-value pairs in the first argument map.
pub fn has_key(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let map = arguments.next().unwrap().must_evaluate(&call_site)?;
    let map = &map.cast_map("first argument")?.map().inner;

    let key = arguments.next().unwrap().must_evaluate(&call_site)?;

    // TODO: check key type?

    Ok(Some(map.contains_key(&key).into()))
}
