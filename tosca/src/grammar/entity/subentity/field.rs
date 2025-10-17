/// Complete field of [Subentity](super::Subentity).
#[macro_export]
macro_rules! complete_subentity_field {
    (
        $field:tt,
        $scope:expr,
        $self:expr,
        $entity:expr,
        $catalog:expr,
        $source_id:expr,
        $errors:expr $(,)?
    ) => {
        match &$entity {
            Some(entity) => match $self.$field.take() {
                Some(mut field) => {
                    field.complete(None, $scope, entity.$field.as_ref(), $catalog, $source_id, $errors.clone())?;
                    $self.$field = Some(field);
                }

                None => {
                    if let Some(field) = &entity.$field {
                        $self.$field = Some(field.into_scoped($scope));
                    }
                }
            },

            None => {
                if let Some(field) = &mut $self.$field {
                    field.complete(None, $scope, None, $catalog, $source_id, $errors.clone())?;
                }
            }
        }
    };
}

/// Complete field of boxed [Subentity](super::Subentity).
#[macro_export]
macro_rules! complete_subentity_boxed_field {
    (
        $field:tt,
        $scope:expr,
        $self:expr,
        $entity:expr,
        $catalog:expr,
        $source_id:expr,
        $errors:expr $(,)?
    ) => {
        match &$entity {
            Some(entity) => match $self.$field.take() {
                Some(mut field) => {
                    field.complete(
                        None,
                        $scope,
                        entity.$field.as_ref().map(|field| field.as_ref()),
                        $catalog,
                        $source_id,
                        $errors.clone(),
                    )?;

                    $self.$field = Some(field);
                }

                None => {
                    if let Some(field) = &entity.$field {
                        $self.$field = Some(field.clone().into_scoped($scope).into());
                    }
                }
            },

            None => {
                if let Some(field) = &mut $self.$field {
                    field.complete(None, $scope, None, $catalog, $source_id, $errors.clone())?;
                }
            }
        }
    };
}

#[allow(unused_imports)]
pub use {complete_subentity_boxed_field, complete_subentity_field};
