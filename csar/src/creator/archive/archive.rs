use super::{
    super::{super::errors::*, compression_level::*, format::*},
    tarball::*,
    zip::*,
};

use {
    indicatif::*,
    std::{io, path::*},
};

/// Default archive entry permissions.
pub const DEFAULT_ARCHIVE_ENTRY_PERMISSIONS: u32 = 0o004;

/// Create an [Archive] file or write to stdout.
///
/// [ZIP](Format::ZIP) requires a file.
pub fn create_archive_file_or_stdout(
    path: Option<&Path>,
    format: Format,
    compression: CompressionLevel,
) -> Result<ArchiveRef, CsarError> {
    match format {
        Format::Tarball => create_tarball_file_or_stdout(path),
        Format::GzipTarball => create_gzip_tarball_file_or_stdout(path, compression),
        Format::ZIP => match path {
            Some(path) => create_zip_file(path),
            None => return Err(CsarError::Missing("ZIP file".into())),
        },
    }
}

//
// Archive
//

/// Common reference type for [Archive].
pub type ArchiveRef = Box<dyn Archive>;

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
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()>;

    /// Create a new archive entry from a file.
    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()>;
}

//
// ArchiveUtilities
//

/// [Archive] utilities.
pub trait ArchiveUtilities {
    /// Create a new archive entry from bytes.
    fn add_bytes<PathT>(
        &mut self,
        name: PathT,
        source: &[u8],
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()>
    where
        PathT: AsRef<Path>;

    /// Create a new archive entry from a string.
    fn add_string<PathT>(
        &mut self,
        name: PathT,
        source: &str,
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()>
    where
        PathT: AsRef<Path>;

    /// Create a new archive entry from a file.
    fn add_file<NamePathT, SourcePathT>(
        &mut self,
        name: NamePathT,
        source: SourcePathT,
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
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
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()>
    where
        PathT: AsRef<Path>,
    {
        self.add_from_reader(
            name.as_ref(),
            Box::new(&mut io::Cursor::new(source)),
            source.len(),
            compression_level,
            progress_bar,
        )
    }

    fn add_string<PathT>(
        &mut self,
        name: PathT,
        source: &str,
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()>
    where
        PathT: AsRef<Path>,
    {
        self.add_bytes(name, source.as_bytes(), compression_level, progress_bar)
    }

    fn add_file<NamePathT, SourcePathT>(
        &mut self,
        name: NamePathT,
        source: SourcePathT,
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()>
    where
        NamePathT: AsRef<Path>,
        SourcePathT: AsRef<Path>,
    {
        self.add_from_file(name.as_ref(), source.as_ref(), compression_level, progress_bar)
    }
}
