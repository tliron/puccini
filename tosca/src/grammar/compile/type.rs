use super::{
    super::{dialect::*, entity::*, name::*, source::*},
    context::*,
    metadata::*,
    name::*,
};

use {compris::annotate::*, problemo::*};

//
// TypeEntityCompiler
//

/// Type entity compiler.
pub struct TypeEntityCompiler<'context, 'compile, DialectT> {
    /// Dialect.
    pub dialect: &'context DialectT,

    /// Dialect ID.
    pub dialect_id: DialectID,

    /// Context.
    pub context: &'context mut CompilationContext<'compile>,
}

impl<'context, 'compile, DialectT> TypeEntityCompiler<'context, 'compile, DialectT> {
    /// Constructor.
    pub fn new(
        dialect: &'context DialectT,
        dialect_id: DialectID,
        context: &'context mut CompilationContext<'compile>,
    ) -> Self {
        Self { dialect, dialect_id, context }
    }

    /// Compile type entity to Floria.
    pub fn compile<EntityTypeT, AnnotatedT>(
        &mut self,
        entity_kind: EntityKind,
        floria_prefix: &str,
        full_name: &FullName,
        source_id: &SourceID,
    ) -> Result<(), Problem>
    where
        EntityTypeT: 'static + TypeEntity<AnnotatedT>,
        AnnotatedT: Annotated + Default + Clone,
        DialectT: Dialect,
    {
        let entity_kind_name = self.dialect.implementation().entity_kinds.represent(entity_kind);

        tracing::debug!(
            source = self.context.source_id.to_string(),
            name = full_name.to_string(),
            type = entity_kind_name.to_string(),
            "compiling"
        );

        let (entity, _source) = give_unwrap!(
            self.context.catalog.entity::<EntityTypeT>(entity_kind, &full_name, self.context.source_id),
            &mut self.context.problems
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

        give_unwrap!(self.context.store.add_class(floria_type), &mut self.context.problems);

        Ok(())
    }
}
