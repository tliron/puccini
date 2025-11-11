/// Complete a field of [FullName](super::super::super::FullName).
#[macro_export]
macro_rules! complete_type_name_field {
    (
        $self:ident,
        $parent:ident,
        $parent_namespace:ident,
        $required:expr,
        $context:expr $(,)?
    ) => {
        if let Some(parent) = &$parent
            && !parent.type_name.is_empty()
        {
            if $self.type_name.is_empty() {
                $self.type_name = parent.type_name.to_namespace($parent_namespace);
            } else {
                validate_type_name(&$self.type_name, &parent.type_name, $context)?;
            }
        }

        if $required && $self.type_name.is_empty() {
            use {::compris::resolve::*, ::kutil::std::error::*};
            $context.errors.give(MissingRequiredKeyError::new("type".into()).with_annotations_from($self))?;
            return Ok(());
        }
    };
}

/// Complete an optional field of [FullName](super::super::super::FullName).
///
/// Only self's field is optional.
#[macro_export]
macro_rules! complete_optional_type_name_field {
    (
        $self:expr,
        $parent:expr,
        $parent_namespace:ident,
        $required:expr,
        $context:expr $(,)?
    ) => {
        if let Some(parent) = &$parent
            && !parent.type_name.is_empty()
        {
            match &$self.type_name {
                None => $self.type_name = Some(parent.type_name.to_namespace($parent_namespace)),
                Some(type_name) => validate_type_name(type_name, &parent.type_name, $context)?,
            }
        }

        if $required && $self.type_name.is_none() {
            use {::compris::resolve::*, ::kutil::std::error::*};
            $context.errors.give(MissingRequiredKeyError::new("type".into()).with_annotations_from($self))?;
            return Ok(());
        }
    };
}

/// Complete an optional field of [FullName](super::super::super::FullName).
///
/// Both self's and parent's fields are optional.
#[macro_export]
macro_rules! complete_optional_parent_type_name_field {
    (
        $field:tt,
        $self:expr,
        $parent:expr,
        $parent_namespace:expr,
        $required:expr,
        $context:expr $(,)?
    ) => {
        if let Some(parent) = &$parent
            && parent.$field.is_some()
        {
            match &$self.$field {
                None => $self.$field = parent.$field.to_namespace($parent_namespace),
                Some(field) => {
                    if let Some(parent_field) = &parent.$field {
                        validate_type_name(field, parent_field, $context)?
                    }
                }
            }
        }

        if $required && $self.$field.is_none() {
            use {::compris::resolve::*, ::kutil::std::error::*};
            $context
                .errors
                .give(MissingRequiredKeyError::new(stringify!($field).into()).with_annotations_from($self))?;
            return Ok(());
        }
    };
}

#[allow(unused_imports)]
pub use {complete_optional_parent_type_name_field, complete_optional_type_name_field, complete_type_name_field};
