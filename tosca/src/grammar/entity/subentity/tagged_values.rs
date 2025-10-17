use super::super::super::{catalog::*, data::*, entity::*, errors::*, name::*, source::*};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

/// Complete tagged values of [Subentity].
#[allow(unused_variables)]
pub fn complete_subentity_tagged_values<SubEntityT, ParentSubEntityT, ErrorRecipientT>(
    type_name: &str,
    scope: Option<&Scope>,
    tagged_values: &mut TaggedValues<ByteString, SubEntityT>,
    parent_tagged_values: Option<&TaggedValues<ByteString, ParentSubEntityT>>,
    must_be_declared: bool,
    catalog: &mut Catalog,
    source_id: &SourceID,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>>
where
    SubEntityT: Annotated + Subentity<ParentSubEntityT>,
    ParentSubEntityT: IntoScoped<SubEntityT>,
    ErrorRecipientT: ErrorRecipient<ToscaError<WithAnnotations>>,
{
    let errors = errors.to_ref();

    // TODO: what if parent has the same name repeated?

    match parent_tagged_values {
        Some(parent_tagged_values) => {
            if must_be_declared {
                for (name, entity) in tagged_values.iter() {
                    if !parent_tagged_values.contains_tag(name) {
                        errors.to_error_recipient().give(
                            UndeclaredError::new(type_name.into(), name.to_string()).with_annotations_from(entity),
                        )?;
                    }
                }
            }

            for (name, parent_entity) in parent_tagged_values {
                match tagged_values.get_first_mut(name) {
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
                        tagged_values.add(name.clone(), entity);
                    }
                }
            }
        }

        None => {
            for (name, entity) in tagged_values {
                entity.complete(Some(name.clone()), scope, None, catalog, source_id, errors.clone())?;
            }
        }
    }

    Ok(())
}
