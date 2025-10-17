use std::str::*;

/// Keys to TOSCA.meta.
pub fn keys_to_tosca_meta<'iter, IteratorT>(iterator: IteratorT, max_columns: Option<usize>) -> String
where
    IteratorT: Iterator<Item = (&'iter String, &'iter String)>,
{
    let mut string = String::default();

    for (key, value) in iterator {
        string += &key_to_tosca_meta(&key, &value, max_columns);
    }

    string
}

/// Key to TOSCA.meta.
pub fn key_to_tosca_meta(key: &str, value: &str, max_columns: Option<usize>) -> String {
    let mut string = String::default();

    let line = format!("{}: {}", key, value);

    let line = match max_columns {
        Some(max_columns) => break_tosca_meta_line(line, max_columns),
        None => line,
    };

    string += &line;
    string += "\n";

    string
}

/// Break line in TOSCA.meta if necessary.
pub fn break_tosca_meta_line(line: String, mut max_columns: usize) -> String {
    if line.len() <= max_columns {
        return line;
    }

    let mut lines = Vec::default();
    let mut chars = line.chars();

    if let Some(line) = next_string(&mut chars, max_columns, false) {
        lines.push(line);
    }

    // Subsequent lines are indented (and thus also shorter)
    max_columns -= 1;
    while let Some(line) = next_string(&mut chars, max_columns, true) {
        lines.push(line);
    }

    lines.join("\n")
}

fn next_string(chars: &mut Chars<'_>, max_count: usize, indent: bool) -> Option<String> {
    let mut line = if indent { String::from(" ") } else { Default::default() };

    let mut count = 0;
    while let Some(c) = chars.next() {
        line.push(c);
        count += 1;
        if count == max_count {
            break;
        }
    }

    let empty = if indent { line.len() == 1 } else { line.is_empty() };
    if !empty { Some(line) } else { None }
}
