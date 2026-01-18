/// Complete a field of a [Taxonomy](super::super::super::Taxonomy) of
/// [Subentity](super::super::super::Subentity).
#[macro_export]
macro_rules! complete_subentity_taxonomy_field {
    (
        $type_name:tt,
        $field:tt,
        $self:expr,
        $entity:expr,
        $entity_namespace:expr,
        $must_be_declared:expr,
        $context:expr $(,)?
    ) => {
        problems_with_fallback_annotations_from_field!(problems, $context.problems, $self, stringify!($field), {
            let context = context_with_problems!($context, problems);
            complete_subentity_taxonomy(
                stringify!($type_name),
                &mut $self.$field,
                $entity.as_ref().map(|entity| &entity.$field),
                $entity_namespace,
                $must_be_declared,
                context,
            )?;
        });
    };
}

#[allow(unused_imports)]
pub use complete_subentity_taxonomy_field;
