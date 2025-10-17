/// Complete schema default and validation.
#[macro_export]
macro_rules! complete_schema_default_and_validation {
    (
        $value_schema:ident,
        $self:ident,
        $definition:ident $(,)?
    ) => {
        if $self.default_expression().is_none()
            && let Some(default) = $definition.default_expression()
        {
            $value_schema.default = Some(default.clone());
        }

        if let Some(validation) = $self.validation() {
            $value_schema.validation.join_and(validation.clone());
        }

        if let Some(validation) = $definition.validation() {
            $value_schema.validation.join_and(validation.clone());
        }
    };
}

/// Complete complex schema default and validation.
#[macro_export]
macro_rules! complete_complex_schema_default_and_validation {
    (
        $value_schema:ident,
        $self:ident,
        $definition:ident $(,)?
    ) => {
        match ($self.default_expression(), $definition.default_expression()) {
            (None, None) => {}
            (Some(default), None) => $value_schema.default = Some(default.clone()),
            (None, Some(default)) => $value_schema.default = Some(default.clone()),
            (Some(default), Some(_)) => {
                return Err($crate::grammar::OverrideProhibitedError::new("default".into())
                    .with_annotations_from(default)
                    .into());
            }
        }

        match ($self.validation(), $definition.validation()) {
            (None, None) => {}
            (Some(validation), None) => $value_schema.validation = Some(validation.clone()),
            (None, Some(validation)) => $value_schema.validation = Some(validation.clone()),
            (Some(validation), Some(_)) => {
                return Err($crate::grammar::OverrideProhibitedError::new("validation".into())
                    .with_annotations_from(validation)
                    .into());
            }
        }
    };
}

/// Complete entry schema.
#[macro_export]
macro_rules! complete_entry_schema {
    (
        $value_schema:ident,
        $self:ident,
        $definition:ident,
        $schema:ident,
        $source_id:ident,
        $catalog:ident $(,)?
    ) => {
        match ($self.entry_schema(), $definition.entry_schema()) {
            (None, None) => {}

            (Some(schema_definition), None) => {
                let reference = schema_definition.initialize_schema($schema, $source_id, $catalog)?;
                $value_schema.entry = Some(reference);
            }

            (None, Some(schema_definition)) => {
                let reference = schema_definition.initialize_schema($schema, $source_id, $catalog)?;
                $value_schema.entry = Some(reference);
            }

            (Some(schema_definition), Some(_)) => {
                return Err($crate::grammar::OverrideProhibitedError::new("entry_schema".into())
                    .with_annotations_from(schema_definition)
                    .into());
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
        $definition:ident,
        $schema:ident,
        $source_id:ident,
        $catalog:ident $(,)?
    ) => {
        match ($self.key_schema(), $definition.key_schema()) {
            (None, None) => {}

            (Some(schema_definition), None) => {
                let reference = schema_definition.initialize_schema($schema, $source_id, $catalog)?;

                // key_schema must be string
                if let Some(schema_definition) = $schema.dereference(reference)
                    && let Some(data_kind) = schema_definition.data_kind()
                    && data_kind != DataKind::String
                {
                    let annotations = $self
                        .key_schema()
                        .and_then(|schema_definition| schema_definition.field_annotations("type_name").cloned());

                    return Err($crate::grammar::WrongTypeError::new(
                        "key_schema".into(),
                        data_kind.to_string(),
                        vec!["string".into()],
                    )
                    .with_annotations_option(annotations)
                    .into());
                }

                $value_schema.key = Some(reference);
            }

            (None, Some(schema_definition)) => {
                let reference = schema_definition.initialize_schema($schema, $source_id, $catalog)?;

                // key_schema must be string kind
                if let Some(schema_definition) = $schema.dereference(reference)
                    && let Some(data_kind) = schema_definition.data_kind()
                    && data_kind != DataKind::String
                {
                    let annotations = $definition
                        .key_schema()
                        .and_then(|schema_definition| schema_definition.field_annotations("type_name").cloned());

                    return Err($crate::grammar::WrongTypeError::new(
                        "key_schema".into(),
                        data_kind.to_string(),
                        vec!["string".into()],
                    )
                    .with_annotations_option(annotations)
                    .into());
                }

                $value_schema.key = Some(reference);
            }

            (Some(schema_definition), Some(_)) => {
                return Err($crate::grammar::OverrideProhibitedError::new("key_schema".into())
                    .with_annotations_from(schema_definition)
                    .into());
            }
        }
    };
}

#[allow(unused_imports)]
pub use {
    complete_complex_schema_default_and_validation, complete_entry_schema, complete_key_schema,
    complete_schema_default_and_validation,
};
