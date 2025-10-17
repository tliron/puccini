use super::{
    super::{super::errors::*, compression_level::*},
    archive::*,
};

use {
    indicatif::*,
    std::{fs::*, io, os::unix::fs::*, path::*, time::*},
    zip::{write::*, *},
};

// Note: The zip crate supports Zopfli, an improved (and slower) DEFLATE algorithm.
// However, we are not enabling it in order to reduce complexity and dependencies.
//
// Moreover, ZIP (and the zip crate) supports other compression methods in addition
// to DEFLATE, however the TOSCA 2.0 spec does not mention compression methods at
// all.
//
// Generally speaking, ZIP is a legacy format for CSAR. Supporting the creation of
// ZIP with basic DEFLATE should be sufficient for this library.
//
// Also note that ZIP uses naive timestamps (no timezone). We use UTC for all
// timestamps but we cannot control how other systems interpret them.

/// Create an [Archive] for a ZIP file.
pub fn create_zip_file(path: &Path) -> Result<ArchiveRef, CsarError> {
    tracing::info!("writing ZIP to {}", path.display());
    let file = Box::new(File::create(path)?);

    let zip_writer = ZipWriter::new(file);
    Ok(Box::new(zip_writer))
}

impl<WriteT> Archive for ZipWriter<WriteT>
where
    WriteT: io::Seek + io::Write,
{
    fn add_from_reader(
        &mut self,
        name: &Path,
        source: Box<&mut dyn io::Read>,
        _size: usize,
        compression: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()> {
        let options = zip_options(DEFAULT_ARCHIVE_ENTRY_PERMISSIONS, SystemTime::now(), compression)?;
        self.start_file_from_path(name, options)?;

        match progress_bar {
            Some(progress_bar) => {
                io::copy(&mut progress_bar.wrap_read(source), self)?;
            }

            None => {
                io::copy(*source, self)?;
            }
        }

        Ok(())
    }

    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        compression: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()> {
        let metadata = source.metadata()?;
        let modified = metadata.modified()?;
        let permissions = metadata.permissions().mode();
        let options = zip_options(permissions, modified, compression)?;

        let mut file = File::open(source)?;
        self.start_file_from_path(name, options)?;

        match progress_bar {
            Some(progress_bar) => {
                io::copy(&mut progress_bar.wrap_read(file), self)?;
            }

            None => {
                io::copy(&mut file, self)?;
            }
        }

        Ok(())
    }
}

// Utils

fn zip_options<'k>(
    permissions: u32,
    modified: SystemTime,
    compression: CompressionLevel,
) -> io::Result<FullFileOptions<'k>> {
    let modified = to_zip_datetime(modified)?;
    Ok(FullFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(Some(compression.to_zip_deflate()))
        .last_modified_time(modified)
        .unix_permissions(permissions))
}

fn to_zip_datetime(system_time: SystemTime) -> io::Result<DateTime> {
    let modified: chrono::DateTime<chrono::Utc> = system_time.into();
    modified.naive_utc().try_into().map_err(io::Error::other)
}
