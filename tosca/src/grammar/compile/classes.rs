use super::super::{super::grammar::*, compile::*, name::*};

use problemo::*;

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
        context: &mut CompilationContext,
    ) -> Result<(), Problem>;
}

impl FloriaToscaType for Vec<floria::ID> {
    fn add_tosca_type(
        &mut self,
        entity_kind: EntityKind,
        entity_kind_name: &str,
        type_name: &FullName,
        context: &mut CompilationContext,
    ) -> Result<(), Problem> {
        if type_name.is_empty() {
            return Ok(());
        }

        let source = give_unwrap!(context.source(), &mut context.problems);
        let type_name = give_unwrap!(
            source.canonical_full_name_for(entity_kind, entity_kind_name, type_name),
            &mut context.problems
        );

        let name = type_name.to_floria_name(entity_kind_name);
        let mut id = floria::ID::new_with_name(floria::EntityKind::Class, context.directory.clone(), name)?;

        loop {
            match give_unwrap!(context.store.get_class(&id), &mut context.problems) {
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
