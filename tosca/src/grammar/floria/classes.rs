use super::super::{errors::*, name::*};

use kutil::std::error::*;

//
// FloriaToscaType
//

/// Add TOSCA type and its ancestors as Floria classes.
pub trait FloriaToscaType {
    /// Add TOSCA type and its ancestors as Floria classes.
    fn add_tosca_type<ErrorRecipientT, AnnotatedT>(
        &mut self,
        type_name: &FullName,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>;
}

impl FloriaToscaType for Vec<floria::ID> {
    fn add_tosca_type<ErrorRecipientT, AnnotatedT>(
        &mut self,
        type_name: &FullName,
        floria_directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let mut id =
            floria::ID::new_for(floria::EntityKind::Class, floria_directory.clone(), type_name.to_string().into());

        loop {
            match unwrap_or_give_and_return!(store.get_class(&id), errors, Ok(())) {
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
