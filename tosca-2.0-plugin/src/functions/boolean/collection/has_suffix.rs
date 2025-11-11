use floria_plugin_sdk::{data::*, errors, utils::*, *};

/// The $has_suffix function takes two arguments. Both arguments are either of type string or of
/// type list. It evaluates to true if the second argument is a suffix of the first argument. For
/// lists this means that the values of the second list are the last values of the first list in
/// the same order.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
pub fn has_suffix(arguments: Vec<Expression>, call_site: CallSite) -> DispatchResult {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let haystack = arguments.next().unwrap().must_evaluate(&call_site)?;
    match haystack {
        Expression::Text(haystack) => {
            let needle = arguments.next().unwrap().must_evaluate(&call_site)?.cast_string("second argument")?;
            Ok(Some(haystack.ends_with(&needle).into()))
        }

        Expression::List(haystack) => {
            let haystack = &haystack.list().inner;
            let needle = arguments.next().unwrap().must_evaluate(&call_site)?;
            let needle = &needle.cast_list("second argument")?.list().inner;

            let needle_length = needle.len();
            let haystack_length = haystack.len();
            if needle_length > haystack_length {
                return Ok(Some(false.into()));
            }

            let start = haystack_length - needle_length;
            for (index, item) in needle.iter().enumerate() {
                if *item != haystack[start + index] {
                    return Ok(Some(false.into()));
                }
            }

            Ok(Some(true.into()))
        }

        _ => Err(errors::not_of_types_for("first argument", &haystack, &["string", "list"])),
    }
}
