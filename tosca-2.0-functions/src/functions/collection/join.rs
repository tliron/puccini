use floria_plugin_sdk::{data::*, utils::*};

/// The $join function takes either one or two arguments where the first one is of type list of
/// strings and the second (optional) argument is of type string. It returns a string that is the
/// joining of the entries in the first argument while adding an optional delimiter between the
/// strings.
pub fn join(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count_range(&arguments, 1, 2)?;
    let mut arguments = arguments.into_iter();

    let list = arguments.next().unwrap().must_evaluate(&call_site)?;
    let list = &list.cast_list("first argument")?.list().inner;

    let delimiter = match arguments.next() {
        Some(delimiter) => {
            let delimiter = delimiter.must_evaluate(&call_site)?.cast_string("second argument")?;
            if !delimiter.is_empty() { Some(delimiter) } else { None }
        }

        None => None,
    };

    let mut joined = String::default();

    let mut iter = list.iter().peekable();
    while let Some(item) = iter.next() {
        let item = item.clone().must_evaluate(&call_site)?.cast_string("list item")?;
        joined.push_str(&item);

        if iter.peek().is_some()
            && let Some(delimiter) = &delimiter
        {
            joined.push_str(delimiter);
        }
    }

    Ok(Some(joined.into()))
}
