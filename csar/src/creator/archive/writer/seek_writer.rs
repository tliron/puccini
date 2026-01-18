use super::{
    super::super::{compression_level::*, format::*},
    writers::*,
};

use {kutil::io::writer::*, problemo::*};

impl ArchiveSeekWriter {
    /// Constructor.
    #[allow(unused_variables)]
    pub fn new_for(
        writer: AnySeekWriterRef,
        format: Format,
        compression_level: Option<CompressionLevel>,
    ) -> Result<Self, Problem> {
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
            _ => {
                use super::super::super::super::errors::*;
                Err(common::UnsupportedError::as_problem("CSAR format").with(format).via(CsarError))
            }
        }
    }
}
