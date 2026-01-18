use super::{
    super::super::{compression_level::*, tracker::*},
    writer::*,
};

use {
    duplicate::*,
    kutil::{io::writer::*, std::any::*},
    self_cell::*,
    std::{any::*, io, path::*},
};

#[duplicate_item(
    ArchiveWriterT        WriterRefT;
    [ArchiveStreamWriter] [AnyWriterRef];
    [ArchiveSeekWriter]   [AnySeekWriterRef];
)]
self_cell!(
    /// An [Archive] that owns its writer.
    pub struct ArchiveWriterT {
        owner: MutBorrow<WriterRefT>,

        #[covariant]
        dependent: ArchiveWriterRef,
    }
);

// Into writer

#[duplicate_item(
    ArchiveWriterT        WriterRefT;
    [ArchiveStreamWriter] [AnyWriterRef];
    [ArchiveSeekWriter]   [AnySeekWriterRef];
)]
impl ArchiveWriterT {
    /// As a writer reference.
    pub fn as_writer_ref(&mut self) -> &mut WriterRefT {
        self.borrow_owner().borrow_mut()
    }

    /// Convert the writer into a concrete type.
    pub fn into_writer<AnyT>(self) -> Option<Box<AnyT>>
    where
        AnyT: Any,
    {
        (*self.into_owner().into_inner()).downcast()
    }
}

// Delegation

#[duplicate_item(
    ArchiveWriterT;
    [ArchiveStreamWriter];
    [ArchiveSeekWriter];
)]
impl ArchiveWriter for ArchiveWriterT {
    fn add_from_reader(
        &mut self,
        name: &Path,
        source: Box<&mut dyn io::Read>,
        size: usize,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()> {
        self.with_dependent_mut(|_writer, archive| {
            archive.add_from_reader(name, source, size, compression_level, read_tracker)
        })
    }

    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()> {
        self.with_dependent_mut(|_writer, archive| archive.add_from_file(name, source, compression_level, read_tracker))
    }
}
