/// Complete a field of [FullName](super::super::super::FullName).
#[macro_export]
macro_rules! complete_name_field {
    (
        $field:tt,
        $self:ident,
        $parent:ident,
        $parent_namespace:ident,
        $context:expr $(,)?
    ) => {
        match &$parent {
            Some(parent) => {
                if parent.$field.is_empty() {
                    if $parent_namespace.is_some() {
                        // $self.$field = $self.$field.to_namespace($parent_namespace);
                    }
                } else {
                    if $self.$field.is_empty() {
                        $self.$field = parent.$field.to_namespace($parent_namespace);
                    } else {
                        if $parent_namespace.is_some() {
                            // $self.$field = $self.$field.to_namespace($parent_namespace);
                        }
                        validate_type_name(&$self.$field, &parent.$field, $context)?;
                    }
                }
            }

            None => {
                if $parent_namespace.is_some() {
                    // $self.$field = $self.$field.to_namespace($parent_namespace);
                }
            }
        }
    };
}

/// Complete an optional field of [FullName](super::super::super::FullName).
///
/// Only self's field is optional.
#[macro_export]
macro_rules! complete_optional_name_field {
    (
        $field:tt,
        $self:expr,
        $parent:expr,
        $parent_namespace:ident,
        $context:expr $(,)?
    ) => {
        match &$parent {
            Some(parent) => {
                if parent.$field.is_empty() {
                    if $parent_namespace.is_some() {
                        // $self.$field = $self.$field.to_namespace($parent_namespace);
                    }
                } else {
                    if $self.$field.is_none() {
                        $self.$field = Some(parent.$field.to_namespace($parent_namespace));
                    } else {
                        if $parent_namespace.is_some() {
                            // $self.$field = $self.$field.to_namespace($parent_namespace);
                        }
                        if let Some(field) = &$self.$field {
                            validate_type_name(field, &parent.$field, $context)?;
                        }
                    }
                }
            }

            None => {
                if $parent_namespace.is_some() {
                    // $self.$field = $self.$field.to_namespace($parent_namespace);
                }
            }
        }
    };
}

/// Complete an optional field of [FullName](super::super::super::FullName).
///
/// Both self's and parent's fields are optional.
#[macro_export]
macro_rules! complete_optional_parent_name_field {
    (
        $field:tt,
        $namespace:expr,
        $self:expr,
        $parent:expr,
        $context:expr $(,)?
    ) => {
        match &$parent {
            Some(parent) => {
                if parent.$field.is_none() {
                    if $namespace.is_some() {
                        // $self.$field = $self.$field.to_namespace($namespace);
                    }
                } else {
                    if $self.$field.is_none() {
                        $self.$field = parent.$field.to_namespace($namespace);
                    } else {
                        if $namespace.is_some() {
                            // $self.$field = $self.$field.to_namespace($namespace);
                        }
                        if let Some(field) = &$self.$field
                            && let Some(parent_field) = &parent.$field
                        {
                            validate_type_name(field, parent_field, $context)?;
                        }
                    }
                }
            }

            None => {
                if $namespace.is_some() {
                    // $self.$field = $self.$field.to_namespace($namespace);
                }
            }
        }
    };
}

#[allow(unused_imports)]
pub use {complete_name_field, complete_optional_name_field, complete_optional_parent_name_field};
