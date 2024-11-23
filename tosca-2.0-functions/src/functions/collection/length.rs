use floria_plugin_sdk::{data::*, errors, utils::*};

/// The $length function takes an argument of type string, list, or map. It returns the number of
/// nicode characters in the string, or the numbers of values in the list, or the number of
/// key-values pairs in the map.
pub fn length(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 1)?;
    let mut arguments = arguments.into_iter();

    let argument = arguments.next().unwrap().must_evaluate(&call_site)?;

    let length = match argument {
        Expression::Text(text) => text.chars().count(),
        Expression::List(list_resource) => list_resource.list().inner.len(),
        Expression::Map(map_resource) => map_resource.map().inner.len(),

        _ => {
            return Err(errors::not_of_types_for("argument", &argument, &["string", "list", "map"]));
        }
    };

    Ok(Some(into_i64(length)?.into()))
}
