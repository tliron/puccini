use super::super::super::{catalog::*, errors::*, name::*, source::*};

use compris::annotate::*;

/// Complete types.
#[allow(unused_variables)]
pub fn complete_type_list<ErrorRecipientT>(
    types: &mut Option<Vec<FullName>>,
    parent_types: &Option<Vec<FullName>>,
    catalog: &mut Catalog,
    source_id: &SourceID,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<WithAnnotations>> {
    Ok(())
}
