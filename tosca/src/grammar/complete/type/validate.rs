use super::super::{
    super::{errors::*, name::*},
    context::*,
};

use compris::annotate::*;

/// Check that our type is the same as or derived from the parent's type.
#[allow(unused_variables)]
pub fn validate_type<TypeT>(
    type_: &TypeT,
    parent_type_name: &FullName,
    context: &mut CompletionContext,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

/// Check that our type is the same as or derived from the parent's type.
#[allow(unused_variables)]
pub fn validate_type_name(
    type_name: &FullName,
    parent_type_name: &FullName,
    context: &mut CompletionContext,
) -> Result<(), ToscaError<WithAnnotations>> {
    // if type_name.is_empty() {
    //     errors.give(ResolveError::from(InvalidKeyError::new("type_name".into())))?;
    // }

    Ok(())
}

/// Call [validate_type_name] on the entity's type.
#[allow(unused_variables)]
pub fn validate_entity_type(
    name: &Name,
    type_names: &Option<Vec<FullName>>,
    context: &mut CompletionContext,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}

/// Check that our type is the same as or derived from the parent's type.
#[allow(unused_variables)]
pub fn validate_entities_types(
    names: &Vec<Name>,
    type_names: &Option<Vec<FullName>>,
    context: &mut CompletionContext,
) -> Result<(), ToscaError<WithAnnotations>> {
    for name in names {
        validate_entity_type(name, type_names, context)?
    }
    Ok(())
}
