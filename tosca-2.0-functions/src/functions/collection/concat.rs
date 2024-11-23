use floria_plugin_sdk::{data::*, errors, utils::*};

/// The $concat function takes one or more arguments of either the type string or the type list
/// with the same type of their entry_schema. In the case of strings, it returns a string which is
/// the concatenation of the argument strings. In the case of lists, it returns a list that
/// contains all the entries of all the argument lists. Order is preserved both for strings and
/// lists. This function does not recurse into the entries of the lists.
pub fn concat(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count_min(&arguments, 1)?;
    let mut arguments = arguments.into_iter();

    while let Some(argument) = arguments.next() {
        let argument = argument.must_evaluate(&call_site)?;
        match argument {
            Expression::Text(text) => {
                let mut concat = text;

                while let Some(argument) = arguments.next() {
                    let argument = argument.must_evaluate(&call_site)?.cast_string("argument")?;
                    concat.push_str(&argument);
                }

                return Ok(Some(concat.into()));
            }

            Expression::List(list_resource) => {
                let mut concat = list_resource.list().clone().inner;

                // TODO: check item type?

                while let Some(argument) = arguments.next() {
                    let argument = argument.must_evaluate(&call_site)?;
                    let argument = &argument.cast_list("argument")?.list().inner;
                    concat.extend_from_slice(argument)
                }

                return Ok(Some(concat.into()));
            }

            _ => return Err(errors::not_of_types_for("argument", &argument, &["string", "list"])),
        }
    }

    Ok(None)
}
