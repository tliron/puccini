use super::super::{catalog::*, data::*, entity::*, errors::*, name::*, source::*};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
    std::collections::*,
};

/// Complete map.
pub fn complete_map<EntityT, ParentEntityT, ErrorRecipientT>(
    type_name: &str,
    map: &mut BTreeMap<ByteString, EntityT>,
    parent_map: Option<(&BTreeMap<ByteString, ParentEntityT>, &Scope)>,
    catalog: &mut Catalog,
    source_id: &SourceID,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    EntityT: Subentity<ParentEntityT> + Annotated,
    ParentEntityT: ConvertIntoScope<EntityT>,
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    let errors = errors.to_ref();

    match parent_map {
        Some((parent_map, scope)) => {
            for (name, entity) in map.iter() {
                if !parent_map.contains_key(name) {
                    errors
                        .to_error_recipient()
                        .give(UndeclaredError::new(type_name.into(), name.to_string()).with_annotations_from(entity))?;
                }
            }

            for (name, parent_entity) in parent_map {
                match map.get_mut(name) {
                    Some(entity) => {
                        entity.complete(
                            Some(name.clone()),
                            Some((parent_entity, scope)),
                            catalog,
                            source_id,
                            errors.clone(),
                        )?;
                    }

                    None => {
                        let mut entity = parent_entity.convert_into_scope(scope);
                        entity.complete(
                            Some(name.clone()),
                            Some((parent_entity, scope)),
                            catalog,
                            source_id,
                            errors.clone(),
                        )?;
                        map.insert(name.clone(), entity);
                    }
                }
            }
        }

        None => {
            for (name, entity) in map {
                entity.complete(Some(name.clone()), None, catalog, source_id, errors.clone())?;
            }
        }
    }

    Ok(())
}

/// Complete tagged values.
#[allow(unused_variables)]
pub fn complete_tagged_values<EntityT, ParentEntityT, ErrorRecipientT>(
    type_name: &str,
    tagged_values: &mut TaggedValues<ByteString, EntityT>,
    parent_tagged_values: Option<(&TaggedValues<ByteString, ParentEntityT>, &Scope)>,
    catalog: &mut Catalog,
    source_id: &SourceID,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    EntityT: Annotated + Subentity<ParentEntityT>,
    ParentEntityT: ConvertIntoScope<EntityT>,
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    let errors = errors.to_ref();

    // TODO: what if parent has the same name repeated?

    match parent_tagged_values {
        Some((parent_tagged_values, scope)) => {
            for (name, entity) in tagged_values.iter() {
                if !parent_tagged_values.contains_tag(name) {
                    errors
                        .to_error_recipient()
                        .give(UndeclaredError::new(type_name.into(), name.to_string()).with_annotations_from(entity))?;
                }
            }

            for (name, parent_entity) in parent_tagged_values {
                match tagged_values.get_first_mut(name) {
                    Some(entity) => {
                        entity.complete(
                            Some(name.clone()),
                            Some((parent_entity, scope)),
                            catalog,
                            source_id,
                            errors.clone(),
                        )?;
                    }

                    None => {
                        let mut entity = parent_entity.convert_into_scope(scope);
                        entity.complete(
                            Some(name.clone()),
                            Some((parent_entity, scope)),
                            catalog,
                            source_id,
                            errors.clone(),
                        )?;
                        tagged_values.add(name.clone(), entity);
                    }
                }
            }
        }

        None => {
            for (name, entity) in tagged_values {
                entity.complete(Some(name.clone()), None, catalog, source_id, errors.clone())?;
            }
        }
    }

    Ok(())
}

/// Complete types.
#[allow(unused_variables)]
pub fn complete_types<ErrorRecipientT>(
    types: &mut Option<Vec<FullName>>,
    parent_types: &Option<Vec<FullName>>,
    catalog: &mut Catalog,
    source_id: &SourceID,
    scope: &Scope,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

/// Complete instances.
#[allow(unused_variables)]
pub fn complete_instances<ErrorRecipientT>(
    instances: &mut Option<Vec<Name>>,
    types: &Option<Vec<FullName>>,
    catalog: &mut Catalog,
    source_id: &SourceID,
    scope: &Scope,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

/// Complete field.
#[macro_export]
macro_rules! complete_field (
    (
        $field:tt,
        $self:ident,
        $entity:ident,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        match &$entity {
            Some((entity, scope)) => {
                match $self.$field.take() {
                    Some(mut field) => {
                        field.complete(
                            None,
                            entity.$field.as_ref().map(|field| (field, *scope)),
                            $catalog,
                            $source_id,
                            $errors.clone(),
                        )?;

                        $self.$field = Some(field);
                    }

                    None => {
                        if let Some(field) = &entity.$field {
                            $self.$field = Some(field.clone().convert_into_scope(scope));
                        }
                    }
                }
            }

            None => {
                if let Some(field) = &mut $self.$field {
                    field.complete(
                        None,
                        None,
                        $catalog,
                        $source_id,
                        $errors.clone(),
                    )?;
                }
            }
        }
    }
);

/// Complete boxed field.
#[macro_export]
macro_rules! complete_boxed_field (
    (
        $field:tt,
        $self:ident,
        $entity:ident,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        match &$entity {
            Some((entity, scope)) => {
                match $self.$field.take() {
                    Some(mut field) => {
                        field.complete(
                            None,
                            entity.$field.as_ref().map(|field| (field.as_ref(), *scope)),
                            $catalog,
                            $source_id,
                            $errors.clone(),
                        )?;

                        $self.$field = Some(field);
                    }

                    None => {
                        if let Some(field) = &entity.$field {
                            $self.$field = Some(field.clone().convert_into_scope(scope).into());
                        }
                    }
                }
            }

            None => {
                if let Some(field) = &mut $self.$field {
                    field.complete(
                        None,
                        None,
                        $catalog,
                        $source_id,
                        $errors.clone(),
                    )?;
                }
            }
        }
    }
);

/// Complete map for field.
#[macro_export]
macro_rules! complete_map_field (
    (
        $type_name:expr,
        $field:tt,
        $self:ident,
        $entity:ident,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        errors_with_fallback_annotations_from_field!(
            $errors, $self, stringify!($field),
            complete_map(
                $type_name,
                &mut $self.$field,
                $entity.as_ref().map(|(entity, scope)| (&entity.$field, *scope)),
                $catalog,
                $source_id,
                $errors,
            )?;
        );
    }
);

/// Complete map for optional field.
#[macro_export]
macro_rules! complete_map_option_field (
    (
        $type_name:expr,
        $field:tt,
        $self:ident,
        $entity:ident,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        let mut field = match $self.$field.take() {
            Some(field) => field,
            None => Default::default(),
        };

        errors_with_fallback_annotations_from_field!(
            $errors, $self, stringify!($field),
            complete_map(
                $type_name,
                &mut field,
                $entity.as_ref().and_then(|(entity, scope)|
                    entity.$field.as_ref().map(|field| (field, *scope))),
                $catalog,
                $source_id,
                $errors,
            )?;
        );

        if !field.is_empty() {
            $self.$field = Some(field);
        }
    }
);

/// Complete tagged values for field.
#[macro_export]
macro_rules! complete_tagged_values_field (
    (
        $type_name:expr,
        $field:tt,
        $self:ident,
        $entity:ident,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        errors_with_fallback_annotations_from_field!(
            $errors, $self, stringify!($field),
            complete_tagged_values(
                $type_name,
                &mut $self.$field,
                $entity.as_ref().map(|(entity, scope)| (&entity.$field, *scope)),
                $catalog,
                $source_id,
                $errors,
            )?;
        );
    }
);

// TODO: Can we avoid cloning? The problem is that we need to later refer to catalog

/// Get complete entity and scope from field.
#[macro_export]
macro_rules! completed_entity (
    (
        $kind:ident,
        $tosca_type:ident,
        $self:ident,
        $field:tt,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        match $catalog
            .completed_entity::<$tosca_type<AnnotatedT>, _, _>(
                $kind,
                &$self.$field,
                $source_id,
                &mut $errors.with_fallback_annotations_from_field($self, stringify!($field)),
            )?
            .cloned()
        {
            Some(entity) => Some((entity, &$self.$field.scope)),
            None => None,
        }
    }
);

/// Get complete entity and scope from field.
#[macro_export]
macro_rules! completed_entity_option (
    (
        $kind:ident,
        $tosca_type:ident,
        $self:ident,
        $field:tt,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        match &$self.$field {
            Some(full_name) => {
                match $catalog
                    .completed_entity::<$tosca_type<AnnotatedT>, _, _>(
                        $kind,
                        full_name,
                        $source_id,
                        &mut $errors.with_fallback_annotations_from_field($self, stringify!($field)),
                    )?
                    .cloned()
                {
                    Some(entity) => Some((entity, &full_name.scope)),
                    None => None,
                }
            }

            None => None,
        }
    }
);

/// Get complete parent and scope from field.
#[macro_export]
macro_rules! completed_parent (
    (
        $kind:ident,
        $self:ident,
        $field:tt,
        $catalog:ident,
        $source_id:ident,
        $derivation_path:ident,
        $errors:ident $(,)?
    ) => {
        match &$self.$field {
            Some(full_name) => {
                match $catalog
                    .completed_entity_with_derivation_path::<Self, _, _>(
                        $kind,
                        full_name,
                        $source_id,
                        $derivation_path,
                        &mut $errors.with_fallback_annotations_from_field($self, stringify!($field)),
                    )?
                    .cloned()
                {
                    Some(parent) => Some((parent, &full_name.scope)),
                    None => None,
                }
            }

            None => None,
        }
    }
);

#[allow(unused_imports)]
pub use {
    complete_boxed_field, complete_field, complete_map_field, complete_map_option_field, complete_tagged_values_field,
    completed_entity, completed_entity_option, completed_parent,
};
