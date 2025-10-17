use super::super::{
    super::{compression_level::*, tracker::*},
    archive::*,
};

use {
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

/// Create a ZIP [Archive].
pub fn new_zip_archive<'own, WriteT>(writer: WriteT) -> ArchiveRef<'own>
where
    WriteT: 'own + io::Seek + io::Write,
{
    Box::new(ZipWriter::new(writer))
}

impl<WriteT> Archive for ZipWriter<WriteT>
where
    WriteT: io::Seek + io::Write,
{
    fn add_from_reader(
        &mut self,
        name: &Path,
        mut source: Box<&mut dyn io::Read>,
        _size: usize,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()> {
        let options = zip_options(DEFAULT_ARCHIVE_ENTRY_PERMISSIONS, SystemTime::now(), compression_level)?;
        add_from_reader(self, name, options, &mut source, read_tracker)
    }

    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()> {
        let metadata = source.metadata()?;
        let modified = metadata.modified()?;
        let permissions = metadata.permissions().mode();
        let options = zip_options(permissions, modified, compression_level)?;

        let mut source = io::BufReader::new(File::open(source)?);
        add_from_reader(self, name, options, &mut source, read_tracker)
    }
}

// Utils

fn add_from_reader<'own, ReadT, WriteT>(
    zip_writer: &mut ZipWriter<WriteT>,
    name: &Path,
    options: FullFileOptions<'own>,
    reader: &mut ReadT,
    read_tracker: Option<&ReadTrackerRef>,
) -> io::Result<()>
where
    ReadT: io::Read,
    WriteT: io::Seek + io::Write,
{
    zip_writer.start_file_from_path(name, options)?;

    match read_tracker {
        Some(read_tracker) => {
            io::copy(&mut read_tracker.track(reader), zip_writer)?;
        }

        None => {
            io::copy(reader, zip_writer)?;
        }
    }

    Ok(())
}

fn zip_options<'own>(
    permissions: u32,
    modified: SystemTime,
    compression_level: Option<CompressionLevel>,
) -> io::Result<FullFileOptions<'own>> {
    let modified = to_zip_datetime(modified)?;

    let compression_level = compression_level.map(|compression_level| compression_level.to_zip_deflate());

    Ok(FullFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .compression_level(compression_level)
        .last_modified_time(modified)
        .unix_permissions(permissions))
}

fn to_zip_datetime(system_time: SystemTime) -> io::Result<DateTime> {
    let modified: chrono::DateTime<chrono::Utc> = system_time.into();
    modified.naive_utc().try_into().map_err(io::Error::other)
}
