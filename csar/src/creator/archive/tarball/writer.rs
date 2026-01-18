use super::{
    super::{super::compression_level::*, writer::*},
    archive::*,
};

use {duplicate::*, kutil::io::writer::*, self_cell::*};

#[duplicate_item(
    ArchiveWriterT        WriterRefT;
    [ArchiveStreamWriter] [AnyWriterRef];
    [ArchiveSeekWriter]   [AnySeekWriterRef];
)]
impl ArchiveWriterT {
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
    ) -> Result<Self, problemo::Problem> {
        Self::try_new(MutBorrow::new(writer), |writer| {
            new_zstandard_tarball_archive(writer.borrow_mut(), compression_level)
        })
    }
}
