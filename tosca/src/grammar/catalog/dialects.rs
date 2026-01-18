use super::{
    super::{dialect::*, entity::*, errors::*},
    catalog::*,
};

use problemo::*;

impl Catalog {
    /// Add a dialect reference.
    pub fn add_dialect_ref(&mut self, dialect: DialectRef) {
        self.dialects.insert(dialect.implementation().dialect_id.clone(), dialect);
    }

    /// Get a dialect reference.
    pub fn get_dialect_ref(&self, dialect_id: &DialectID) -> Result<&DialectRef, Problem> {
        self.dialects.get(dialect_id).ok_or_else(|| UnsupportedDialectError::as_problem(dialect_id.clone()))
    }

    /// Get a [Dialect].
    pub fn get_dialect<DialectT>(&self, dialect_id: &DialectID) -> Result<&DialectT, Problem>
    where
        DialectT: 'static,
    {
        let dialect = self.get_dialect_ref(dialect_id)?;
        dialect.downcast_ref_checked()
    }

    /// Supported entity kinds.
    pub fn dialect_entity_kinds(&self, dialect_id: &DialectID) -> Result<&EntityKinds, Problem> {
        Ok(&self.get_dialect_ref(dialect_id)?.implementation().entity_kinds)
    }
}
