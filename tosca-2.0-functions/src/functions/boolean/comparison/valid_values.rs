use floria_plugin_sdk::{data::*, utils::*, *};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $valid_values function takes two arguments. The first argument is of any type and the
/// second argument is a list with any number of values of the same type as the first argument. It
/// evaluates to true if the first argument is equal to a value in the second argument list and
/// false otherwise.
pub fn valid_values(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let needle = arguments.next().unwrap().must_evaluate(&call_site)?;

    let haystack = arguments.next().unwrap().must_evaluate(&call_site)?;
    let haystack = &haystack.cast_list("second argument")?.list().inner;

    // TODO: check needle type?

    Ok(Some(haystack.contains(&needle).into()))
}
