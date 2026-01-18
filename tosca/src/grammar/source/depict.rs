use super::{super::entity::*, source::*};

use {depiction::*, kutil::std::iter::*, std::io};

impl Source {
    /// To namespaces depiction.
    pub fn namespaces_depiction<'this, 'kinds, 'context>(
        &'this self,
        entity_kinds: &'kinds EntityKinds,
    ) -> NamespacesDepiction<'context>
    where
        'this: 'context,
        'kinds: 'context,
    {
        NamespacesDepiction::new(self, entity_kinds)
    }

    /// To entities depiction.
    pub fn entities_depiction<'this, 'kinds, 'context>(
        &'this self,
        entity_kinds: &'kinds EntityKinds,
    ) -> EntitiesDepiction<'context>
    where
        'this: 'context,
        'kinds: 'context,
    {
        EntitiesDepiction::new(self, entity_kinds)
    }
}

//
// NamespacesDepiction
//

/// Namespaces depiction.
pub struct NamespacesDepiction<'context> {
    /// Source.
    pub source: &'context Source,

    /// Entity kinds.
    pub entity_kinds: &'context EntityKinds,
}

impl<'context> NamespacesDepiction<'context> {
    /// Constructor.
    pub fn new(source: &'context Source, entity_kinds: &'context EntityKinds) -> Self {
        Self { source, entity_kinds }
    }
}

impl<'context> Depict for NamespacesDepiction<'context> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for ((entity_kind, names), last) in IterateWithLast::new(self.source.namespace_tree()) {
            let entity_kind_name = self.entity_kinds.represent(entity_kind);
            context.indent_into_branch(writer, last)?;
            context.theme.write_heading(writer, entity_kind_name)?;

            let context = context.child().increase_indentation_branch(last);
            for ((full_name, source_id), last) in IterateWithLast::new(names) {
                context.indent_into_branch(writer, last)?;

                if !full_name.is_empty() {
                    write!(writer, "{} ", context.theme.name(full_name))?;
                }

                write!(writer, "{}{}", context.theme.delimiter(DEPICT_LOCATION_PREFIX), context.theme.meta(source_id))?;
            }
        }

        Ok(())
    }
}

//
// EntitiesDepiction
//

/// Entities depiction.
pub struct EntitiesDepiction<'context> {
    /// Source.
    pub source: &'context Source,

    /// Entity kinds.
    pub entity_kinds: &'context EntityKinds,
}

impl<'context> EntitiesDepiction<'context> {
    /// Constructor.
    pub fn new(source: &'context Source, entity_kinds: &'context EntityKinds) -> Self {
        Self { source, entity_kinds }
    }
}

impl<'context> Depict for EntitiesDepiction<'context> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        for ((entity_kind, names), last) in IterateWithLast::new(self.source.entity_names_tree()) {
            let entity_kind_name = self.entity_kinds.represent(entity_kind);
            context.indent_into_branch(writer, last)?;
            context.theme.write_heading(writer, entity_kind_name)?;

            let context = context.child().increase_indentation_branch(last).with_configuration("heading", "false");
            for (name, last) in IterateWithLast::new(names) {
                let entity = self.source.entities.get(&WithEntityKind::new(entity_kind, name.clone())).expect("entity");

                context.indent_into_branch(writer, last)?;
                context.theme.write_name(writer, name)?;
                entity.depict(writer, &context.child().increase_indentation_branch(last))?;
            }
        }

        Ok(())
    }
}
