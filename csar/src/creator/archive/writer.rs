use super::{
    super::{super::errors::*, compression_level::*, format::*, tracker::*},
    archive::*,
};

use {
    duplicate::*,
    kutil::{io::writer::*, std::any::*},
    self_cell::*,
    std::{any::*, io, path::*},
};

//
// ArchiveWriter
//

#[duplicate_item(
    ArchiveT            WriterRefT;
    [ArchiveWriter]     [AnyWriterRef];
    [ArchiveSeekWriter] [AnySeekWriterRef];
)]
self_cell!(
    /// An [Archive] that owns its writer.
    pub struct ArchiveT {
        owner: MutBorrow<WriterRefT>,

        #[covariant]
        dependent: ArchiveRef,
    }
);

// Constructors

impl ArchiveWriter {
    /// Constructor.
    ///
    /// [ZIP](Format::ZIP) is not supported. Use [ArchiveSeekWriter::new_for] instead.
    #[allow(unused_variables)]
    pub fn new_for(
        writer: AnyWriterRef,
        format: Format,
        compression_level: Option<CompressionLevel>,
    ) -> Result<Self, CsarError> {
        match format {
            #[cfg(feature = "tarball")]
            Format::Tarball => Ok(Self::new_tarball(writer)),

            #[cfg(all(feature = "tarball", feature = "gzip"))]
            Format::GzipTarball => Ok(Self::new_gzip_tarball(writer, compression_level)),

            #[cfg(all(feature = "tarball", feature = "zstandard"))]
            Format::ZstandardTarball => Self::new_zstandard_tarball(writer, compression_level),

            #[cfg(feature = "zip")]
            Format::ZIP => Err(CsarError::Invalid("cannot create archive for ZIP".into())),

            #[cfg(not(all(feature = "tarball", feature = "gzip", feature = "zstandard", feature = "zip")))]
            _ => Err(CsarError::UnsupportedFormat(format)),
        }
    }
}

impl ArchiveSeekWriter {
    /// Constructor.
    #[allow(unused_variables)]
    pub fn new_for(
        writer: AnySeekWriterRef,
        format: Format,
        compression_level: Option<CompressionLevel>,
    ) -> Result<Self, CsarError> {
        match format {
            #[cfg(feature = "tarball")]
            Format::Tarball => Ok(Self::new_tarball(writer)),

            #[cfg(all(feature = "tarball", feature = "gzip"))]
            Format::GzipTarball => Ok(Self::new_gzip_tarball(writer, compression_level)),

            #[cfg(all(feature = "tarball", feature = "zstandard"))]
            Format::ZstandardTarball => Self::new_zstandard_tarball(writer, compression_level),

            #[cfg(feature = "zip")]
            Format::ZIP => Ok(Self::new_zip(writer)),

            #[cfg(not(all(feature = "tarball", feature = "gzip", feature = "zstandard", feature = "zip")))]
            _ => Err(CsarError::UnsupportedFormat(format)),
        }
    }
}

// Into writer

#[duplicate_item(
    ArchiveT;
    [ArchiveWriter];
    [ArchiveSeekWriter];
)]
impl ArchiveT {
    /// Convert the writer into a concrete type.
    pub fn into_writer<AnyT>(self) -> Result<Box<AnyT>, Box<dyn Any>>
    where
        AnyT: Any,
    {
        Box::leak(self.into_owner().into_inner()).into_concrete()
    }
}

// Delegation

#[duplicate_item(
    ArchiveT;
    [ArchiveWriter];
    [ArchiveSeekWriter];
)]
impl Archive for ArchiveT {
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
