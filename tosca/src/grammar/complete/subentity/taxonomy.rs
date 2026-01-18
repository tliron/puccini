use super::super::{
    super::{data::*, entity::*, errors::*, name::*},
    context::*,
};

use {compris::annotate::*, problemo::*};

/// Complete a [Taxonomy] of [Subentity].
#[allow(unused_variables)]
pub fn complete_subentity_taxonomy<SubentityT, ParentSubentityT>(
    type_name: &str,
    taxonomy: &mut Taxonomy<Name, SubentityT>,
    parent_taxonomy: Option<&Taxonomy<Name, ParentSubentityT>>,
    parent_namespace: Option<&Namespace>,
    must_be_declared: bool,
    context: &mut CompletionContext,
) -> Result<(), Problem>
where
    SubentityT: Annotated + Subentity<ParentSubentityT>,
    ParentSubentityT: ToNamespace<SubentityT>,
{
    // TODO: what if parent has the same name repeated?

    match parent_taxonomy {
        Some(parent_taxonomy) => {
            if must_be_declared {
                for (name, subentity) in taxonomy.iter() {
                    if !parent_taxonomy.contains_name(name) {
                        context
                            .problems
                            .give(UndeclaredError::as_problem(type_name, name).with_annotations_from(subentity))?;
                    }
                }
            }

            for (name, parent_subentity) in parent_taxonomy {
                match taxonomy.first_mut(name) {
                    Some(subentity) => {
                        subentity.complete(Some(name), Some(parent_subentity), parent_namespace, context)?;
                    }

                    None => {
                        let mut subentity = parent_subentity.to_namespace(parent_namespace);
                        subentity.complete(Some(name), Some(parent_subentity), parent_namespace, context)?;
                        taxonomy.add(name.clone(), subentity);
                    }
                }
            }

            for (name, subentity) in taxonomy {
                if !parent_taxonomy.contains_name(name) {
                    subentity.complete(Some(name), None, None, context)?;
                }
            }
        }

        None => {
            for (name, subentity) in taxonomy {
                subentity.complete(Some(name), None, None, context)?;
            }
        }
    }

    Ok(())
}
