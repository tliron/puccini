use super::{
    super::{
        super::{super::errors::*, compression_level::*},
        writer::*,
    },
    archive::*,
};

use {duplicate::*, kutil::io::writer::*, self_cell::*};

#[duplicate_item(
    ArchiveT            WriterRefT;
    [ArchiveWriter]     [AnyWriterRef];
    [ArchiveSeekWriter] [AnySeekWriterRef];
)]
impl ArchiveT {
    /// Constructor.
    pub fn new_tarball(writer: WriterRefT) -> Self {
        Self::new(MutBorrow::new(writer), |writer| new_tarball_archive(writer.borrow_mut()))
    }

    /// Constructor.
    #[cfg(feature = "gzip")]
    pub fn new_gzip_tarball(writer: WriterRefT, compression_level: Option<CompressionLevel>) -> Self {
        Self::new(MutBorrow::new(writer), |writer| new_gzip_tarball_archive(writer.borrow_mut(), compression_level))
    }

    /// Constructor.
    #[cfg(feature = "zstandard")]
    pub fn new_zstandard_tarball(
        writer: WriterRefT,
        compression_level: Option<CompressionLevel>,
    ) -> Result<Self, CsarError> {
        Self::try_new(MutBorrow::new(writer), |writer| {
            new_zstandard_tarball_archive(writer.borrow_mut(), compression_level)
        })
    }
}
