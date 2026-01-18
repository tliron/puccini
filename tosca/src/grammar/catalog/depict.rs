use super::{super::entity::*, catalog::*};

use {depiction::*, kutil::std::iter::*, std::io};

impl Catalog {
    /// To namespaces depiction.
    pub fn namespaces_depiction<'this>(&'this self, skip_internal: bool) -> NamespacesDepiction<'this> {
        NamespacesDepiction::new(self, skip_internal)
    }

    /// To entities depiction.
    pub fn entities_depiction<'this>(&'this self, skip_internal: bool) -> EntitiesDepiction<'this> {
        EntitiesDepiction::new(self, skip_internal)
    }
}

//
// NamespacesDepiction
//

/// Namespaces depiction.
pub struct NamespacesDepiction<'inner> {
    /// Inner catalog.
    pub inner: &'inner Catalog,

    /// Whether to skip internal sources.
    pub skip_internal: bool,
}

impl<'inner> NamespacesDepiction<'inner> {
    /// Constructor.
    pub fn new(inner: &'inner Catalog, skip_internal: bool) -> Self {
        Self { inner, skip_internal }
    }
}

impl<'inner> Depict for NamespacesDepiction<'inner> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let default_entity_kinds = EntityKinds::default();

        for ((source_id, source), first) in IterateWithFirst::new(IterateByKeyOrder::new(&self.inner.sources)) {
            if self.skip_internal && source_id.is_internal() {
                continue;
            }

            let entity_kinds = self.inner.dialect_entity_kinds(&source.dialect_id).unwrap_or(&default_entity_kinds);

            if !first {
                writeln!(writer)?;
                context.indent(writer)?;
            }

            context.theme.write_heading(writer, source_id)?;
            source.namespaces_depiction(entity_kinds).depict(writer, context)?;
        }

        Ok(())
    }
}

//
// EntitiesDepiction
//

/// Entities depiction.
pub struct EntitiesDepiction<'inner> {
    /// Inner catalog.
    pub inner: &'inner Catalog,

    /// Whether to skip internal sources.
    pub skip_internal: bool,
}

impl<'inner> EntitiesDepiction<'inner> {
    /// Constructor.
    pub fn new(inner: &'inner Catalog, skip_internal: bool) -> Self {
        Self { inner, skip_internal }
    }
}

impl<'inner> Depict for EntitiesDepiction<'inner> {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        let default_entity_kinds = EntityKinds::default();

        for ((source_id, source), first) in IterateWithFirst::new(IterateByKeyOrder::new(&self.inner.sources)) {
            if self.skip_internal && source_id.is_internal() {
                continue;
            }

            let entity_kinds = self.inner.dialect_entity_kinds(&source.dialect_id).unwrap_or(&default_entity_kinds);

            if !first {
                writeln!(writer)?;
                context.indent(writer)?;
            }

            context.theme.write_heading(writer, source_id)?;
            source.entities_depiction(entity_kinds).depict(writer, context)?;
        }

        Ok(())
    }
}
