use super::{
    super::{super::errors::*, compression_level::*, format::*},
    any::*,
    archive::Archive,
};

use {
    duplicate::*,
    flate2::{write::*, *},
    indicatif::*,
    kutil::std::any::*,
    self_cell::*,
    std::{any::*, io, path::*},
    tar::*,
    zip::write::*,
};

//
// ArchiveWithWriter
//

#[duplicate_item(
    ArchiveT                WriterRefT;
    [ArchiveWithWriter]     [AnyWriterRef];
    [ArchiveWithSeekWriter] [AnySeekWriterRef];
)]
self_cell!(
    /// An [Archive] that owns its writer.
    pub struct ArchiveT {
        owner: MutBorrow<WriterRefT>,

        #[covariant]
        dependent: DependentArchive,
    }
);

// self_cell needs a non-nested type name
type DependentArchive<'own> = Box<dyn Archive + 'own>;

#[duplicate_item(
    ArchiveT                WriterRefT;
    [ArchiveWithWriter]     [AnyWriterRef];
    [ArchiveWithSeekWriter] [AnySeekWriterRef];
)]
impl ArchiveT {
    /// Constructor.
    pub fn new_for_tarball(writer: WriterRefT) -> Self {
        Self::new(MutBorrow::new(writer), |writer| {
            let writer = writer.borrow_mut();
            let tar_builder = Builder::new(writer);
            Box::new(tar_builder)
        })
    }

    /// Constructor.
    pub fn new_for_gzip_tarball(writer: WriterRefT, compression: CompressionLevel) -> Self {
        Self::new(MutBorrow::new(writer), |writer| {
            let writer = writer.borrow_mut();
            let gz_encoder = GzEncoder::new(writer, Compression::new(compression.to_gzip()));
            let tar_builder = Builder::new(gz_encoder);
            Box::new(tar_builder)
        })
    }

    /// Convert the writer into a concrete type.
    pub fn into_writer<AnyT>(self) -> Result<Box<AnyT>, Box<dyn Any>>
    where
        AnyT: Any,
    {
        Box::leak(self.into_owner().into_inner()).into_concrete()
    }
}

impl ArchiveWithWriter {
    /// Constructor.
    ///
    /// [ZIP](Format::ZIP) is not supported. Use [ArchiveWithSeekWriter::new_for] instead.
    pub fn new_for(writer: AnyWriterRef, format: Format, compression: CompressionLevel) -> Result<Self, CsarError> {
        match format {
            Format::Tarball => Ok(Self::new_for_tarball(writer)),
            Format::GzipTarball => Ok(Self::new_for_gzip_tarball(writer, compression)),
            Format::ZIP => Err(CsarError::Invalid("cannot create archive for ZIP".into())),
        }
    }
}

impl ArchiveWithSeekWriter {
    /// Constructor.
    pub fn new_for(writer: AnySeekWriterRef, format: Format, compression: CompressionLevel) -> Result<Self, CsarError> {
        Ok(match format {
            Format::Tarball => Self::new_for_tarball(writer),
            Format::GzipTarball => Self::new_for_gzip_tarball(writer, compression),
            Format::ZIP => Self::new_for_zip(writer),
        })
    }

    /// Constructor.
    pub fn new_for_zip(writer: AnySeekWriterRef) -> Self {
        Self::new(MutBorrow::new(writer), |writer| {
            let writer = writer.borrow_mut();
            let zip_writer = ZipWriter::new(writer);
            Box::new(zip_writer)
        })
    }
}

// Delegation

#[duplicate_item(
    ArchiveT;
    [ArchiveWithWriter];
    [ArchiveWithSeekWriter];
)]
impl Archive for ArchiveT {
    fn add_from_reader(
        &mut self,
        name: &Path,
        source: Box<&mut dyn io::Read>,
        size: usize,
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()> {
        self.with_dependent_mut(|_owner, dependent| {
            dependent.add_from_reader(name, source, size, compression_level, progress_bar)
        })
    }

    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        compression_level: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()> {
        self.with_dependent_mut(|_owner, dependent| {
            dependent.add_from_file(name, source, compression_level, progress_bar)
        })
    }
}
