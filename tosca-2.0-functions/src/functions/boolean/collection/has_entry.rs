use floria_plugin_sdk::{data::*, errors, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $has_entry function takes two arguments. The first argument is a list or a map. The second
/// argument is of the type matching the entry_schema of the first argument. It evaluates to true
/// if the second argument is an entry in the first argument. For lists this means that the second
/// argument is a value in the first argument list. For maps this means that the second argument is
/// a value in any of the key-value pairs in the first argument map.
pub fn has_entry(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let haystack = arguments.next().unwrap().must_evaluate(&call_site)?;
    match haystack {
        Expression::List(haystack) => {
            let haystack = &haystack.list().inner;
            let needle = arguments.next().unwrap().must_evaluate(&call_site)?;

            // TODO: check needle type?

            Ok(Some(haystack.contains(&needle).into()))
        }

        Expression::Map(haystack) => {
            let haystack = &haystack.map().inner;
            let needle = arguments.next().unwrap().must_evaluate(&call_site)?;

            // TODO: check needle type?

            for value in haystack.values() {
                if needle == *value {
                    return Ok(Some(true.into()));
                }
            }

            Ok(Some(false.into()))
        }

        _ => Err(errors::not_of_types_for("first argument", &haystack, &["list", "map"])),
    }
}
