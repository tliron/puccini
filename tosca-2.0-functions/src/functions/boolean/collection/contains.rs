use floria_plugin_sdk::{data::*, errors, utils::*};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// The $contains function takes two arguments. Both arguments are either of type string or of type
/// list. It evaluates to true if the second argument is contained in the first argument. For
/// strings that means that the second argument is a substring of the first argument. For lists
/// this means that the values of the second list are contained in the first list in an
/// uninterrupted sequence and in the same order.
pub fn contains(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 2)?;
    let mut arguments = arguments.into_iter();

    let haystack = arguments.next().unwrap().must_evaluate(&call_site)?;
    match haystack {
        Expression::Text(haystack) => {
            let needle = arguments.next().unwrap().must_evaluate(&call_site)?.cast_string("second argument")?;
            Ok(Some(haystack.contains(&needle).into()))
        }

        Expression::List(haystack) => {
            let haystack = &haystack.list().inner;
            let needle = arguments.next().unwrap().must_evaluate(&call_site)?;

            // TODO: check needle type?

            Ok(Some(haystack.contains(&needle).into()))
        }

        _ => Err(errors::not_of_types_for("first argument", &haystack, &["string", "list"])),
    }
}
