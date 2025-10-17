use super::super::super::{catalog::*, entity::*, errors::*, name::*, source::*};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
    std::collections::*,
};

/// Complete map of [Subentity].
pub fn complete_subentity_map<SubEntityT, ParentSubEntityT, ErrorRecipientT>(
    type_name: &str,
    scope: Option<&Scope>,
    map: &mut BTreeMap<ByteString, SubEntityT>,
    parent_map: Option<&BTreeMap<ByteString, ParentSubEntityT>>,
    must_be_declared: bool,
    catalog: &mut Catalog,
    source_id: &SourceID,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    SubEntityT: Subentity<ParentSubEntityT> + Annotated,
    ParentSubEntityT: IntoScoped<SubEntityT>,
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    let errors = errors.to_ref();

    match parent_map {
        Some(parent_map) => {
            if must_be_declared {
                for (name, entity) in map.iter() {
                    if !parent_map.contains_key(name) {
                        errors.to_error_recipient().give(
                            UndeclaredError::new(type_name.into(), name.to_string()).with_annotations_from(entity),
                        )?;
                    }
                }
            }

            for (name, parent_entity) in parent_map {
                match map.get_mut(name) {
                    Some(entity) => {
                        entity.complete(
                            Some(name.clone()),
                            scope,
                            Some(parent_entity),
                            catalog,
                            source_id,
                            errors.clone(),
                        )?;
                    }

                    None => {
                        let mut entity = parent_entity.into_scoped(scope);
                        entity.complete(
                            Some(name.clone()),
                            scope,
                            Some(parent_entity),
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
                entity.complete(Some(name.clone()), scope, None, catalog, source_id, errors.clone())?;
            }
        }
    }

    Ok(())
}
