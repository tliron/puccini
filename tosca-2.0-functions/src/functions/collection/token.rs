use floria_plugin_sdk::{data::*, utils::*};

/// The $token function is used within a TOSCA service template on a string to parse out (tokenize)
/// substrings separated by one or more token characters within a larger string.
pub fn token(arguments: Vec<Expression>, call_site: CallSite) -> Result<Option<Expression>, String> {
    assert_argument_count(&arguments, 3)?;
    let mut arguments = arguments.into_iter();

    let string = arguments.next().unwrap().must_evaluate(&call_site)?.cast_string("first argument")?;

    let delimiters = arguments.next().unwrap().must_evaluate(&call_site)?.cast_string("second argument")?;
    if delimiters.is_empty() {
        return Err("second argument empty".into());
    }

    let index = arguments.next().unwrap().must_evaluate(&call_site)?.cast_i64("third argument")?;

    let mut split = string.split(|c| delimiters.contains(c));

    let mut i = 0;
    while let Some(token) = split.next() {
        if i == index {
            return Ok(Some(token.into()));
        }

        i += 1;
    }

    Err(format!("token not found: |error|{}|", index))
}
