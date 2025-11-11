/// Complete a field of a [Subentity](super::super::super::Subentity).
#[macro_export]
macro_rules! complete_subentity_field {
    (
        $field:tt,
        $self:expr,
        $parent:expr,
        $parent_namespace:expr,
        $context:expr $(,)?
    ) => {
        match &$parent {
            Some(parent) => match $self.$field.take() {
                Some(mut subentity) => {
                    subentity.complete(None, parent.$field.as_ref(), $parent_namespace, $context)?;
                    $self.$field = Some(subentity);
                }

                None => {
                    if let Some(parent_subentity) = &parent.$field {
                        let mut subentity = parent_subentity.to_namespace($parent_namespace);
                        subentity.complete(None, Some(parent_subentity), $parent_namespace, $context)?;
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
        $parent:expr,
        $parent_namespace:expr,
        $context:expr $(,)?
    ) => {
        match &$parent {
            Some(parent) => match $self.$field.take() {
                Some(mut subentity) => {
                    let parent_subentity = parent.$field.as_ref().map(|subentity| subentity.as_ref());
                    subentity.complete(None, parent_subentity, $parent_namespace, $context)?;
                    $self.$field = Some(subentity);
                }

                None => {
                    if let Some(parent_subentity) = &parent.$field {
                        let mut subentity = parent_subentity.to_namespace($parent_namespace);
                        subentity.complete(None, Some(parent_subentity), $parent_namespace, $context)?;
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
