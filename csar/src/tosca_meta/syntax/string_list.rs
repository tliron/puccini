use super::super::super::errors::*;

/// String list to TOSCA.meta.
pub fn string_list_to_tosca_meta(keyname: &str, values: &Vec<String>) -> Result<Vec<String>, CsarError> {
    values.iter().map(|value| coerce_list_value(keyname, value)).collect()
}

/// String list from TOSCA.meta.
pub fn string_list_from_tosca_meta(keyname: &str, value: &str) -> Result<Vec<String>, CsarError> {
    let mut list = Vec::default();

    let mut current = String::default();
    let mut mode = ListMode::Normal;
    let mut iter = value.chars();
    while let Some(c) = iter.next() {
        match c {
            ' ' => match mode {
                ListMode::Normal => {
                    if current.is_empty() {
                        // Caused by having more than one space in sequence
                        return Err(MalformedKeyError::new(keyname.into(), "has empty list item".into()).into());
                    }

                    list.push(current.clone());
                    current.clear();
                }

                ListMode::Quoted => current.push(c),
            },

            '\"' => match mode {
                ListMode::Normal => {
                    if !current.is_empty() {
                        return Err(MalformedKeyError::new(keyname.into(), "has `\"` in a list item".into()).into());
                    }

                    // Start quoted value
                    mode = ListMode::Quoted;
                }

                ListMode::Quoted => {
                    // If anything follows, it *must* be a space
                    if let Some(c) = iter.next() {
                        if c != ' ' {
                            return Err(MalformedKeyError::new(keyname.into(), "has `\"` in a list item".into()).into());
                        }
                    }

                    if current.is_empty() {
                        return Err(MalformedKeyError::new(keyname.into(), "has empty list item".into()).into());
                    }

                    list.push(current.clone());
                    current.clear();

                    mode = ListMode::Normal;
                }
            },

            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        if matches!(mode, ListMode::Quoted) {
            return Err(MalformedKeyError::new(keyname.into(), "has opening `\"` with no closing".into()).into());
        }

        list.push(current);
    }

    Ok(list)
}

enum ListMode {
    Normal,
    Quoted,
}

fn coerce_list_value(keyname: &str, string: &String) -> Result<String, CsarError> {
    if string.contains('\"') {
        return Err(InvalidKeyError::new(keyname.into(), "list value contains `\"`".into()).into());
    }

    Ok(if string.contains(' ') { String::from("\"") + string + "\"" } else { string.clone() })
}
