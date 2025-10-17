use super::{super::writer::*, archive::*};

use {kutil::io::writer::*, self_cell::*};

impl ArchiveSeekWriter {
    /// Constructor.
    pub fn new_zip(writer: AnySeekWriterRef) -> Self {
        Self::new(MutBorrow::new(writer), |writer| new_zip_archive(writer.borrow_mut()))
    }
}
