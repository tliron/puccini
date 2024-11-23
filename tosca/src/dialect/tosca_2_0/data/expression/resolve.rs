use super::{call::*, expression::*};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::std::{error::*, immutable::*},
    std::collections::*,
};

/// Function prefix.
pub const FUNCTION_PREFIX: &str = "$";

impl<AnnotatedT> Expression<AnnotatedT> {
    fn resolve_list<ErrorRecipientT>(
        list: List<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut expression_list = Vec::with_capacity(list.inner.len());

        for item in list {
            if let Some(item) = item.resolve_with_errors(errors)? {
                expression_list.push(item);
            }
        }

        let mut simple = true;
        for item in &expression_list {
            if !matches!(item, Expression::Simple(_)) {
                simple = false;
                break;
            }
        }

        Ok(Some(if simple {
            Expression::Simple(
                expression_list
                    .into_iter()
                    .map(|item| {
                        let Expression::Simple(item) = item else { panic!("should be simple") };
                        item
                    })
                    .collect(),
            )
        } else {
            Self::List(expression_list)
        }))
    }

    fn resolve_map<ErrorRecipientT>(
        map: Map<AnnotatedT>,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Self, AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        let mut expression_map = BTreeMap::default();

        for pair in map {
            if let Some((key, value)) = pair.resolve_with_errors(errors)? {
                expression_map.insert(key, value);
            }
        }

        let mut simple = true;
        for (key, value) in &expression_map {
            if !matches!(key, Expression::Simple(_)) || !matches!(value, Expression::Simple(_)) {
                simple = false;
                break;
            }
        }

        Ok(Some(if simple {
            Expression::Simple(
                expression_map
                    .into_iter()
                    .map(|(key, value)| {
                        let Expression::Simple(key) = key else { panic!("should be simple") };
                        let Expression::Simple(value) = value else { panic!("should be simple") };
                        (key, value)
                    })
                    .collect(),
            )
        } else {
            Self::Map(expression_map)
        }))
    }
}

impl<AnnotatedT> Resolve<Expression<AnnotatedT>, AnnotatedT> for Variant<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn resolve_with_errors<ErrorRecipientT>(
        self,
        errors: &mut ErrorRecipientT,
    ) -> ResolveResult<Expression<AnnotatedT>, AnnotatedT>
    where
        ErrorRecipientT: ErrorRecipient<ResolveError<AnnotatedT>>,
    {
        match self {
            Variant::Text(text) => {
                // Call with no arguments
                if text.inner.starts_with(FUNCTION_PREFIX) {
                    let string = &text.inner[1..];

                    // Escaped?
                    if string.starts_with(FUNCTION_PREFIX) {
                        // Unescape
                        let unescaped = Variant::from(ByteString::from(string)).with_annotations_from(&text);
                        return Ok(Some(unescaped.into()));
                    }

                    return Ok(Some(
                        Call::new_native(
                            Text::from(ByteString::from(string)).with_annotations_from(&text),
                            Default::default(),
                            floria::CallKind::Normal,
                        )
                        .into(),
                    ));
                }

                Ok(Some(text.inner.into()))
            }

            Variant::List(list) => Expression::resolve_list(list, errors),

            Variant::Map(map) => {
                let prefixed = if let Some((key, _)) = map.to_key_value_pair()
                    && let Variant::Text(key_text) = key
                    && key_text.inner.starts_with(FUNCTION_PREFIX)
                {
                    true
                } else {
                    false
                };

                if prefixed {
                    let (key, value) = map.inner.into_iter().next().expect("single-key map");
                    let Variant::Text(key_text) = key else { panic!("key should be text") };

                    let key_string = &key_text.inner[1..];

                    // Escaped?
                    if key_string.starts_with(FUNCTION_PREFIX) {
                        // Unescape
                        let unescaped_key =
                            Variant::from(ByteString::from(key_string)).with_annotations_from(&key_text);
                        return Expression::resolve_map(Map::from([(unescaped_key, value)]), errors);
                    }

                    let mut arguments = Vec::default();
                    // TODO: are we allowing non-list arguments?
                    for argument in value.into_iterator() {
                        let argument = argument.resolve_with_errors(errors)?.unwrap_or_default();
                        arguments.push(argument);
                    }

                    return Ok(Some(
                        Call::new_native(
                            Text::from(ByteString::from(key_string)).with_annotations_from(&key_text),
                            arguments,
                            floria::CallKind::Normal,
                        )
                        .into(),
                    ));
                }

                Expression::resolve_map(map, errors)
            }

            _ => Ok(Some(self.into())),
        }
    }
}
