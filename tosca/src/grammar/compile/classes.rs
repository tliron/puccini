use super::super::{super::grammar::*, compile::*, errors::*, name::*};

use {compris::annotate::*, kutil::std::error::*};

//
// FloriaToscaType
//

/// Add TOSCA type and its ancestors as Floria classes.
pub trait FloriaToscaType {
    /// Add TOSCA type and its ancestors as Floria classes.
    fn add_tosca_type(
        &mut self,
        entity_kind: EntityKind,
        entity_kind_name: &str,
        type_name: &FullName,
        context: &mut CompilationContext<'_>,
    ) -> Result<(), ToscaError<WithAnnotations>>;
}

impl FloriaToscaType for Vec<floria::ID> {
    fn add_tosca_type(
        &mut self,
        entity_kind: EntityKind,
        entity_kind_name: &str,
        type_name: &FullName,
        context: &mut CompilationContext<'_>,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        if type_name.is_empty() {
            return Ok(());
        }

        let source = must_unwrap_give!(context.source(), context.errors);
        let type_name =
            must_unwrap_give!(source.canonical_full_name_for(entity_kind, entity_kind_name, type_name), context.errors);

        let name = type_name.to_floria_name(entity_kind_name);
        let mut id = floria::ID::new_with_name(floria::EntityKind::Class, context.directory.clone(), name)?;

        loop {
            match must_unwrap_give!(context.store.get_class(&id), context.errors) {
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
