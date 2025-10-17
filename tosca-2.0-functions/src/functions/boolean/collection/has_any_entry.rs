use floria_plugin_sdk::{data::*, errors, utils::*, *};

/// The $has_any_entry function takes two arguments. The first argument is a list or a map. The
/// second argument is a list with the entry_schema matching the entry_schema of the first
/// argument. It evaluates to true if there is an entry in the second argument that is equal to an
/// entry in the first argument.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn has_any_entry(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let container = arguments.next().unwrap().must_evaluate(&call_site)?;
    match container {
        Expression::List(list) => {
            let list = &list.list().inner;
            let entries = arguments.next().unwrap().must_evaluate(&call_site)?;
            let entries = &entries.cast_list("second argument")?.list().inner;

            for item in entries {
                // TODO: check entry type?

                if list.contains(item) {
                    return Ok(Some(true.into()));
                }
            }

            Ok(Some(false.into()))
        }

        Expression::Map(map) => {
            let map = &map.map().inner;
            let entries = arguments.next().unwrap().must_evaluate(&call_site)?;
            let entries = &entries.cast_list("second argument")?.list().inner;

            for item in entries {
                // TODO: check entry type?

                for value in map.values() {
                    if value == item {
                        return Ok(Some(true.into()));
                    }
                }
            }

            Ok(Some(false.into()))
        }

        _ => Err(errors::not_of_types_for("first argument", &container, &["list", "map"])),
    }
}
