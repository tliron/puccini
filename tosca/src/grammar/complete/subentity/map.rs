use super::super::{
    super::{entity::*, errors::*, name::*},
    context::*,
};
use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
    std::collections::*,
};

/// Complete a map of [Subentity].
pub fn complete_subentity_map<SubentityT, ParentSubentityT>(
    type_name: &str,
    map: &mut BTreeMap<ByteString, SubentityT>,
    parent_map: Option<&BTreeMap<ByteString, ParentSubentityT>>,
    parent_namespace: Option<&Namespace>,
    must_be_declared: bool,
    context: &mut CompletionContext,
) -> Result<(), ToscaError<WithAnnotations>>
where
    SubentityT: Annotated + Subentity<ParentSubentityT>,
    ParentSubentityT: ToNamespace<SubentityT>,
{
    match parent_map {
        Some(parent_map) => {
            if must_be_declared {
                for (name, subentity) in map.iter() {
                    if !parent_map.contains_key(name) {
                        context.errors.give(
                            UndeclaredError::new(type_name.into(), name.to_string()).with_annotations_from(subentity),
                        )?;
                    }
                }
            }

            for (name, parent_subentity) in parent_map {
                match map.get_mut(name) {
                    Some(subentity) => {
                        subentity.complete(Some(name.clone()), Some(parent_subentity), parent_namespace, context)?;
                    }

                    None => {
                        let mut subentity = parent_subentity.to_namespace(parent_namespace);
                        subentity.complete(Some(name.clone()), None, None, context)?;
                        map.insert(name.clone(), subentity);
                    }
                }
            }
        }

        None => {
            for (name, subentity) in map {
                subentity.complete(Some(name.clone()), None, None, context)?;
            }
        }
    }

    Ok(())
}
