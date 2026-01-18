use super::{
    super::super::{compression_level::*, tracker::*},
    writer::*,
    writers::*,
};

use std::{any::*, io, path::*};

//
// ArchiveWriterWrapper
//

/// Either an [ArchiveStreamWriter] or an [ArchiveSeekWriter].
pub enum ArchiveWriterWrapper {
    /// Stream writer.
    Stream(ArchiveStreamWriter),

    /// Seek writer.
    Seek(ArchiveSeekWriter),
}

impl ArchiveWriterWrapper {
    /// Convert the writer into a concrete type.
    pub fn into_writer<AnyT>(self) -> Option<Box<AnyT>>
    where
        AnyT: Any,
    {
        match self {
            Self::Stream(writer) => writer.into_writer(),
            Self::Seek(writer) => writer.into_writer(),
        }
    }

    /// Flush.
    pub fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Stream(writer) => writer.as_writer_ref().flush(),
            Self::Seek(writer) => writer.as_writer_ref().flush(),
        }
    }
}

impl ArchiveWriter for ArchiveWriterWrapper {
    fn add_from_reader(
        &mut self,
        name: &Path,
        source: Box<&mut dyn io::Read>,
        size: usize,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()> {
        match self {
            Self::Stream(writer) => writer.add_from_reader(name, source, size, compression_level, read_tracker),
            Self::Seek(writer) => writer.add_from_reader(name, source, size, compression_level, read_tracker),
        }
    }

    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()> {
        match self {
            Self::Stream(writer) => writer.add_from_file(name, source, compression_level, read_tracker),
            Self::Seek(writer) => writer.add_from_file(name, source, compression_level, read_tracker),
        }
    }
}

impl From<ArchiveStreamWriter> for ArchiveWriterWrapper {
    fn from(writer: ArchiveStreamWriter) -> Self {
        Self::Stream(writer)
    }
}

impl From<ArchiveSeekWriter> for ArchiveWriterWrapper {
    fn from(writer: ArchiveSeekWriter) -> Self {
        Self::Seek(writer)
    }
}
