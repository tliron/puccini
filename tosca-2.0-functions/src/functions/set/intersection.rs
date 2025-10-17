use floria_plugin_sdk::{data::*, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $intersection function takes one or more list arguments, all having the entry schema of the
/// same type. The result is a list that contains all entries that can be found in each of the
/// argument lists.
pub fn intersection(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count_min(&arguments, 1)?;

    let mut lists = Vec::default();
    for argument in arguments {
        let argument = argument.must_evaluate(&call_site)?;
        let argument = argument.cast_list("argument")?.list().inner.clone();
        lists.push(argument);
    }

    let mut intersection = Vec::default();

    // TODO: do we need to evaluate the items?
    // TODO: this could be more efficient; we are doing the same comparisons more than once

    for (index, list) in lists.iter().enumerate() {
        for item in list {
            let mut in_all_others = true;

            for (other_index, other_list) in lists.iter().enumerate() {
                if (index != other_index) && !other_list.contains(item) {
                    in_all_others = false;
                    break;
                }
            }

            if in_all_others {
                intersection.push(item.clone());
            }
        }
    }

    Ok(Some(intersection.into()))
}
