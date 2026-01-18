/// Complete schema default and validation.
#[macro_export]
macro_rules! complete_schema_default_and_validation {
    (
        $value_schema:expr,
        $self:expr,
        $details:expr,
        $details_namespace:expr $(,)?
    ) => {
        if let Some(default) = $self.default_expression() {
            $value_schema.default = Some(default.clone());
        } else if let Some(default) = $details.default_expression() {
            $value_schema.default = Some(default.to_namespace($details_namespace));
        }

        if let Some(validation) = $self.validation() {
            $value_schema.validation.join_and(validation.clone());
        }

        if let Some(validation) = $details.validation() {
            $value_schema.validation.join_and(validation.to_namespace($details_namespace));
        }
    };
}

/// Complete complex schema default and validation.
#[macro_export]
macro_rules! complete_complex_schema_default_and_validation {
    (
        $value_schema:expr,
        $self:expr,
        $details:expr,
        $details_namespace:expr,
        $context:expr $(,)?
    ) => {
        match ($self.default_expression(), $details.default_expression()) {
            (None, None) => {}
            (Some(default), None) => $value_schema.default = Some(default.clone()),
            (None, Some(default)) => $value_schema.default = Some(default.to_namespace($details_namespace)),
            (Some(default), Some(_)) => {
                use {::compris::annotate::*, ::problemo::*};
                $context.problems.give(
                    $crate::grammar::OverrideProhibitedError::as_problem("default").with_annotations_from(default),
                )?;
                return Ok(None);
            }
        }

        match ($self.validation(), $details.validation()) {
            (None, None) => {}
            (Some(validation), None) => $value_schema.validation = Some(validation.clone()),
            (None, Some(validation)) => $value_schema.validation = Some(validation.to_namespace($details_namespace)),
            (Some(validation), Some(_)) => {
                use {::compris::annotate::*, ::problemo::*};
                $context.problems.give(
                    $crate::grammar::OverrideProhibitedError::as_problem("validation")
                        .with_annotations_from(validation),
                )?;
                return Ok(None);
            }
        }
    };
}

/// Complete entry schema.
#[macro_export]
macro_rules! complete_entry_schema {
    (
        $value_schema:expr,
        $self:expr,
        $details:expr,
        $details_namespace:expr,
        $schema:expr,
        $context:expr $(,)?
    ) => {
        match ($self.entry_schema(), $details.entry_schema()) {
            (None, None) => {}

            (Some(schema_definition), None) => {
                if let Some(reference) = schema_definition.initialize_schema($schema, $context)? {
                    $value_schema.entry = Some(reference);
                }
            }

            (None, Some(schema_definition)) => {
                if let Some(reference) =
                    schema_definition.to_namespace($details_namespace).initialize_schema($schema, $context)?
                {
                    $value_schema.entry = Some(reference);
                }
            }

            (Some(schema_definition), Some(_)) => {
                use {::compris::annotate::*, ::problemo::*};
                $context.problems.give(
                    $crate::grammar::OverrideProhibitedError::as_problem("entry_schema")
                        .with_annotations_from(schema_definition),
                )?;
                return Ok(None);
            }
        }
    };
}

/// Complete key schema.
#[macro_export]
macro_rules! complete_key_schema {
    (
        $value_schema:ident,
        $self:ident,
        $details:ident,
        $details_namespace:expr,
        $schema:ident,
        $context:ident $(,)?
    ) => {
        match ($self.key_schema(), $details.key_schema()) {
            (None, None) => {}

            (Some(schema_definition), None) => {
                if let Some(reference) = schema_definition.initialize_schema($schema, $context)? {
                    // key_schema must be string
                    if let Some(schema_definition) = $schema.dereference(reference)
                        && let Some(data_kind) = schema_definition.data_kind()
                        && data_kind != DataKind::String
                    {
                        let annotations = $self
                            .key_schema()
                            .and_then(|schema_definition| schema_definition.field_annotations("type_name").cloned());

                        use {::compris::annotate::*, ::problemo::*};
                        $context.problems.give(
                            $crate::grammar::WrongTypeError::as_problem("key_schema", data_kind, vec!["string".into()])
                                .with_annotations_option(annotations),
                        )?;
                        return Ok(None);
                    }

                    $value_schema.key = Some(reference);
                }
            }

            (None, Some(schema_definition)) => {
                if let Some(reference) =
                    schema_definition.to_namespace($details_namespace).initialize_schema($schema, $context)?
                {
                    // key_schema must be string kind
                    if let Some(schema_definition) = $schema.dereference(reference)
                        && let Some(data_kind) = schema_definition.data_kind()
                        && data_kind != DataKind::String
                    {
                        let annotations = $details
                            .key_schema()
                            .and_then(|schema_definition| schema_definition.field_annotations("type_name").cloned());

                        use {::compris::annotate::*, ::problemo::*};
                        $context.problems.give(
                            $crate::grammar::WrongTypeError::as_problem("key_schema", data_kind, vec!["string".into()])
                                .with_annotations_option(annotations),
                        )?;
                        return Ok(None);
                    }

                    $value_schema.key = Some(reference);
                }
            }

            (Some(schema_definition), Some(_)) => {
                use {::compris::annotate::*, ::problemo::*};
                $context.problems.give(
                    $crate::grammar::OverrideProhibitedError::as_problem("key_schema")
                        .with_annotations_from(schema_definition),
                )?;
                return Ok(None);
            }
        }
    };
}

#[allow(unused_imports)]
pub use {
    complete_complex_schema_default_and_validation, complete_entry_schema, complete_key_schema,
    complete_schema_default_and_validation,
};
