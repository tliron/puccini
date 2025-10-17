use super::{
    super::{dialect::*, entity::*, errors::*, name::*, source::*},
    context::*,
    metadata::*,
};

use {compris::annotate::*, kutil::std::error::*};

/// Compile type entity to Floria.
pub fn compile_type<EntityTypeT, DialectT>(
    entity_kind: EntityKind,
    full_name: &FullName,
    entity_source_id: &SourceID,
    dialect: &DialectT,
    dialect_id: DialectID,
    context: &mut CompilationContext<'_>,
) -> Result<(), ToscaError<WithAnnotations>>
where
    EntityTypeT: 'static + TypeEntity<WithAnnotations>,
    DialectT: Dialect,
{
    let entity_kind_name = dialect.entity_kinds().represent(entity_kind);

    tracing::debug!(
        source = context.source_id.to_string(),
        name = full_name.to_string(),
        type = entity_kind_name.to_string(),
        "compiling"
    );

    let descriptor = unwrap_or_give_and_return!(
        context.catalog.entity::<EntityTypeT, _>(entity_kind, &full_name, context.source_id),
        context.errors,
        Ok(())
    )
    .descriptor();

    let mut floria_type = ::floria::Class::new_for(context.directory.clone(), full_name.to_string().into());

    floria_type.metadata.set_tosca_entity(dialect_id, entity_kind_name.clone());
    floria_type.metadata.set_tosca_version(descriptor.version.as_ref());
    floria_type.metadata.set_tosca_description(descriptor.description);
    floria_type.metadata.set_tosca_custom_metadata(descriptor.metadata);

    if matches!(entity_source_id, SourceID::Internal(_)) {
        floria_type.metadata.set_tosca_internal(true);
    }

    unwrap_or_give!(context.store.add_class(floria_type), context.errors);

    Ok(())
}
