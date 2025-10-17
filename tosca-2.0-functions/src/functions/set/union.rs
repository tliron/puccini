use floria_plugin_sdk::{data::*, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $union function takes one or more list arguments, all having the entry schema of the same
/// type. The result is a list that contains all non-duplicate entries from all the argument lists.
/// By non-duplicate is meant that no two entries in the result list are equal.
pub fn union(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count_min(&arguments, 1)?;

    let mut union = Vec::default();

    for argument in arguments {
        let argument = argument.must_evaluate(&call_site)?;
        let argument = &argument.cast_list("argument")?.list().inner;

        // TODO: check entry type?

        for item in argument {
            if !union.contains(item) {
                union.push(item.clone());
            }
        }
    }

    Ok(Some(union.into()))
}
