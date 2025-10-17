use super::super::{
    super::{errors::*, name::*},
    context::*,
};

use compris::annotate::*;

/// Complete a list of type names.
#[allow(unused_variables)]
pub fn complete_type_list(
    types: &mut Option<Vec<FullName>>,
    parent_types: &Option<Vec<FullName>>,
    context: &mut CompletionContext,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}
