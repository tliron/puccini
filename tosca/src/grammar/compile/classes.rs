use super::super::{compile::*, errors::*, name::*};

use {compris::annotate::*, kutil::std::error::*};

//
// FloriaToscaType
//

/// Add TOSCA type and its ancestors as Floria classes.
pub trait FloriaToscaType {
    /// Add TOSCA type and its ancestors as Floria classes.
    fn add_tosca_type(
        &mut self,
        type_name: &FullName,
        context: &mut CompilationContext<'_>,
    ) -> Result<(), ToscaError<WithAnnotations>>;
}

impl FloriaToscaType for Vec<floria::ID> {
    fn add_tosca_type(
        &mut self,
        type_name: &FullName,
        context: &mut CompilationContext<'_>,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let mut id =
            floria::ID::new_for(floria::EntityKind::Class, context.directory.clone(), type_name.to_string().into());

        loop {
            match unwrap_or_give_and_return!(context.store.get_class(&id), context.errors, Ok(())) {
                Some(class) => {
                    self.push(class.id.clone());
                    // TODO: we don't have tosca:parent
                    match class.metadata.inner.get(&"tosca:parent".into()) {
                        Some(parent) => id.name = parent.to_string().into(),
                        None => break,
                    }
                }

                None => {
                    // TODO
                    break;
                }
            }
        }

        Ok(())
    }
}
