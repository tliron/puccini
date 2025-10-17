use super::super::{compression_level::*, tracker::*};

use std::{io, path::*};

/// Default archive entry permissions.
pub const DEFAULT_ARCHIVE_ENTRY_PERMISSIONS: u32 = 0o004;

//
// Archive
//

/// Common reference type for [Archive].
pub type ArchiveRef<'own> = Box<dyn Archive + 'own>;

/// Archive.
///
/// This trait is `dyn`-compatible.
pub trait Archive {
    /// Create a new archive entry from a reader.
    fn add_from_reader(
        &mut self,
        name: &Path,
        source: Box<&mut dyn io::Read>,
        size: usize,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()>;

    /// Create a new archive entry from a file.
    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()>;
}

//
// ArchiveUtilities
//

// We can't add these functions directly to the Archive trait because it must be `dyn`-compatible.

/// [Archive] utilities.
pub trait ArchiveUtilities {
    /// Create a new archive entry from bytes.
    fn add_bytes<PathT>(
        &mut self,
        name: PathT,
        source: &[u8],
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()>
    where
        PathT: AsRef<Path>;

    /// Create a new archive entry from a string.
    fn add_string<PathT>(
        &mut self,
        name: PathT,
        source: &str,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()>
    where
        PathT: AsRef<Path>;

    /// Create a new archive entry from a file.
    fn add_file<NamePathT, SourcePathT>(
        &mut self,
        name: NamePathT,
        source: SourcePathT,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()>
    where
        NamePathT: AsRef<Path>,
        SourcePathT: AsRef<Path>;
}

impl<ArchiveT> ArchiveUtilities for ArchiveT
where
    ArchiveT: Archive + ?Sized,
{
    fn add_bytes<PathT>(
        &mut self,
        name: PathT,
        source: &[u8],
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()>
    where
        PathT: AsRef<Path>,
    {
        self.add_from_reader(
            name.as_ref(),
            Box::new(&mut io::Cursor::new(source)),
            source.len(),
            compression_level,
            read_tracker,
        )
    }

    fn add_string<PathT>(
        &mut self,
        name: PathT,
        source: &str,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()>
    where
        PathT: AsRef<Path>,
    {
        self.add_bytes(name, source.as_bytes(), compression_level, read_tracker)
    }

    fn add_file<NamePathT, SourcePathT>(
        &mut self,
        name: NamePathT,
        source: SourcePathT,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()>
    where
        NamePathT: AsRef<Path>,
        SourcePathT: AsRef<Path>,
    {
        self.add_from_file(name.as_ref(), source.as_ref(), compression_level, read_tracker)
    }
}
