use super::super::{
    super::{entity::*, errors::*, name::*},
    context::*,
};
use {compris::annotate::*, problemo::*, std::collections::*};

/// Complete a map of [Subentity].
pub fn complete_subentity_map<SubentityT, ParentSubentityT>(
    type_name: &str,
    map: &mut BTreeMap<Name, SubentityT>,
    parent_map: Option<&BTreeMap<Name, ParentSubentityT>>,
    parent_namespace: Option<&Namespace>,
    must_be_declared: bool,
    context: &mut CompletionContext,
) -> Result<(), Problem>
where
    SubentityT: Annotated + Subentity<ParentSubentityT>,
    ParentSubentityT: ToNamespace<SubentityT>,
{
    match parent_map {
        Some(parent_map) => {
            if must_be_declared {
                for (name, subentity) in map.iter() {
                    if !parent_map.contains_key(name) {
                        context
                            .problems
                            .give(UndeclaredError::as_problem(type_name, name).with_annotations_from(subentity))?;
                    }
                }
            }

            for (name, parent_subentity) in parent_map {
                match map.get_mut(name) {
                    Some(subentity) => {
                        subentity.complete(Some(name), Some(parent_subentity), parent_namespace, context)?;
                    }

                    None => {
                        let mut subentity = parent_subentity.to_namespace(parent_namespace);
                        subentity.complete(Some(name), Some(parent_subentity), parent_namespace, context)?;
                        map.insert(name.clone(), subentity);
                    }
                }
            }

            for (name, subentity) in map {
                if !parent_map.contains_key(name) {
                    subentity.complete(Some(name), None, None, context)?;
                }
            }
        }

        None => {
            for (name, subentity) in map {
                subentity.complete(Some(name), None, None, context)?;
            }
        }
    }

    Ok(())
}
