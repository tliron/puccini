use super::{
    super::{dialect::*, entity::*, errors::*, name::*, source::*},
    context::*,
    metadata::*,
    name::*,
};

use {compris::annotate::*, kutil::std::error::*};

//
// TypeEntityCompiler
//

/// Type entity compiler.
pub struct TypeEntityCompiler<'own, 'context, DialectT> {
    /// Dialect.
    pub dialect: &'own DialectT,

    /// Dialect ID.
    pub dialect_id: DialectID,

    /// Context.
    pub context: &'own mut CompilationContext<'context>,
}

impl<'own, 'context, DialectT> TypeEntityCompiler<'own, 'context, DialectT> {
    /// Constructor.
    pub fn new(
        dialect: &'own DialectT,
        dialect_id: DialectID,
        context: &'own mut CompilationContext<'context>,
    ) -> Self {
        Self { dialect, dialect_id, context }
    }

    /// Compile type entity to Floria.
    pub fn compile<EntityTypeT>(
        &mut self,
        entity_kind: EntityKind,
        floria_prefix: &str,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<(), ToscaError<WithAnnotations>>
    where
        EntityTypeT: 'static + TypeEntity<WithAnnotations>,
        DialectT: Dialect,
    {
        let entity_kind_name = self.dialect.entity_kinds().represent(entity_kind);

        tracing::debug!(
            source = self.context.source_id.to_string(),
            name = full_name.to_string(),
            type = entity_kind_name.to_string(),
            "compiling"
        );

        let (entity, _source) = must_unwrap_give!(
            self.context.catalog.entity::<EntityTypeT, _>(entity_kind, &full_name, self.context.source_id),
            self.context.errors
        );
        let descriptor = entity.descriptor();

        let mut floria_type =
            ::floria::Class::new_with_name(self.context.directory.clone(), full_name.to_floria_name(floria_prefix))?;

        floria_type.metadata.set_tosca_entity(self.dialect_id.clone(), entity_kind_name.clone());
        floria_type.metadata.set_tosca_version(descriptor.version.as_ref());
        floria_type.metadata.set_tosca_description(descriptor.description);
        floria_type.metadata.set_tosca_custom_metadata(descriptor.metadata);

        if matches!(source_id, SourceID::Internal(_)) {
            floria_type.metadata.set_tosca_internal(true);
        }

        unwrap_or_give!(self.context.store.add_class(floria_type), self.context.errors);

        Ok(())
    }
}
