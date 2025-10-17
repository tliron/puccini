/// Complete a field of a [Subentity](super::super::super::Subentity).
#[macro_export]
macro_rules! complete_subentity_field {
    (
        $field:tt,
        $self:expr,
        $entity:expr,
        $entity_namespace:expr,
        $context:expr $(,)?
    ) => {
        match &$entity {
            Some(entity) => match $self.$field.take() {
                Some(mut subentity) => {
                    subentity.complete(None, entity.$field.as_ref(), $entity_namespace, $context)?;
                    $self.$field = Some(subentity);
                }

                None => {
                    if let Some(subentity) = &entity.$field {
                        let mut subentity = subentity.to_namespace($entity_namespace);
                        subentity.complete(None, None, None, $context)?;
                        $self.$field = Some(subentity);
                    }
                }
            },

            None => {
                if let Some(subentity) = &mut $self.$field {
                    subentity.complete(None, None, None, $context)?;
                }
            }
        }
    };
}

/// Complete a field of a boxed [Subentity](super::super::super::Subentity).
#[macro_export]
macro_rules! complete_boxed_subentity_field {
    (
        $field:tt,
        $self:expr,
        $entity:expr,
        $entity_namespace:expr,
        $context:expr $(,)?
    ) => {
        match &$entity {
            Some(entity) => match $self.$field.take() {
                Some(mut subentity) => {
                    subentity.complete(
                        None,
                        entity.$field.as_ref().map(|subentity| subentity.as_ref()),
                        $entity_namespace,
                        $context,
                    )?;

                    $self.$field = Some(subentity);
                }

                None => {
                    if let Some(subentity) = &entity.$field {
                        let mut subentity = subentity.to_namespace($entity_namespace);
                        subentity.complete(None, None, None, $context)?;
                        $self.$field = Some(subentity.into());
                    }
                }
            },

            None => {
                if let Some(subentity) = &mut $self.$field {
                    subentity.complete(None, None, None, $context)?;
                }
            }
        }
    };
}

#[allow(unused_imports)]
pub use {complete_boxed_subentity_field, complete_subentity_field};
