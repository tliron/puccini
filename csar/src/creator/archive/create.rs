use super::{
    super::{super::errors::*, compression_level::*, format::*},
    writer::*,
};

use {
    problemo::*,
    std::{fs::*, io, path::*},
};

/// Create an [Archive] file or write to stdout.
///
/// [ZIP](Format::ZIP) *requires* a file.
#[allow(unused_variables)]
pub fn create_archive_file_or_stdout(
    path: Option<&Path>,
    format: Format,
    compression_level: Option<CompressionLevel>,
) -> Result<ArchiveWriterRef<'_>, Problem> {
    let writer = || -> Result<Box<dyn io::Write + Send>, Problem> {
        Ok(match path {
            Some(path) => Box::new(io::BufWriter::new(File::create(path)?)),
            None => Box::new(io::stdout()),
        })
    };

    match format {
        #[cfg(feature = "tarball")]
        Format::Tarball => Ok(super::tarball::new_tarball_archive(writer()?)),

        #[cfg(all(feature = "tarball", feature = "gzip"))]
        Format::GzipTarball => Ok(super::tarball::new_gzip_tarball_archive(writer()?, compression_level)),

        #[cfg(all(feature = "tarball", feature = "zstandard"))]
        Format::ZstandardTarball => super::tarball::new_zstandard_tarball_archive(writer()?, compression_level),

        #[cfg(feature = "zip")]
        Format::ZIP => match path {
            Some(path) => Ok(super::zip::new_zip_archive(Box::new(File::create(path)?))),
            None => Err(common::MissingError::as_problem("ZIP file").via(CsarError)),
        },

        #[cfg(not(all(feature = "tarball", feature = "gzip", feature = "zstandard", feature = "zip")))]
        _ => Err(common::UnsupportedError::as_problem("CSAR format").with(format).via(CsarError)),
    }
}
