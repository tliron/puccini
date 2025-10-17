/// Complete field of [FullName](super::FullName).
#[macro_export]
macro_rules! complete_name_field {
    (
        $field:tt,
        $scope:ident,
        $self:ident,
        $parent:ident,
        $catalog:ident,
        $errors:ident $(,)?
    ) => {
        match &$parent {
            Some(parent) => {
                if parent.$field.is_empty() {
                    if $scope.is_some() {
                        $self.$field = $self.$field.into_scoped($scope);
                    }
                } else {
                    if $self.$field.is_empty() {
                        $self.$field = parent.$field.into_scoped($scope);
                    } else {
                        if $scope.is_some() {
                            $self.$field = $self.$field.into_scoped($scope);
                        }
                        validate_type_name(&$self.$field, &parent.$field, $catalog, $errors)?;
                    }
                }
            }

            None => {
                if $scope.is_some() {
                    $self.$field = $self.$field.into_scoped($scope);
                }
            }
        }
    };
}

/// Complete field of [FullName](super::FullName).
///
/// Self's field is optional.
#[macro_export]
macro_rules! complete_name_field_self_option {
    (
        $field:tt,
        $scope:expr,
        $self:expr,
        $parent:expr,
        $catalog:expr,
        $errors:expr $(,)?
    ) => {
        match &$parent {
            Some(parent) => {
                if parent.$field.is_empty() {
                    if $scope.is_some() {
                        $self.$field = $self.$field.into_scoped($scope);
                    }
                } else {
                    if $self.$field.is_none() {
                        $self.$field = Some(parent.$field.into_scoped($scope));
                    } else {
                        if $scope.is_some() {
                            $self.$field = $self.$field.into_scoped($scope);
                        }
                        if let Some(field) = &$self.$field {
                            validate_type_name(field, &parent.$field, $catalog, $errors)?;
                        }
                    }
                }
            }

            None => {
                if $scope.is_some() {
                    $self.$field = $self.$field.into_scoped($scope);
                }
            }
        }
    };
}

/// Complete field of [FullName](super::FullName).
///
/// Both self's and parent's fields are optional.
#[macro_export]
macro_rules! complete_name_field_both_option {
    (
        $field:tt,
        $scope:expr,
        $self:expr,
        $parent:expr,
        $catalog:expr,
        $errors:expr $(,)?
    ) => {
        match &$parent {
            Some(parent) => {
                if parent.$field.is_none() {
                    if $scope.is_some() {
                        $self.$field = $self.$field.into_scoped($scope);
                    }
                } else {
                    if $self.$field.is_none() {
                        $self.$field = parent.$field.into_scoped($scope);
                    } else {
                        if $scope.is_some() {
                            $self.$field = $self.$field.into_scoped($scope);
                        }
                        if let Some(field) = &$self.$field
                            && let Some(parent_field) = &parent.$field
                        {
                            validate_type_name(field, parent_field, $catalog, $errors)?;
                        }
                    }
                }
            }

            None => {
                if $scope.is_some() {
                    $self.$field = $self.$field.into_scoped($scope);
                }
            }
        }
    };
}

#[allow(unused_imports)]
pub use {complete_name_field, complete_name_field_both_option, complete_name_field_self_option};
