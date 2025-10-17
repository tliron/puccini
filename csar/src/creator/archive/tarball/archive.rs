use super::super::{
    super::{super::errors::*, compression_level::*, tracker::*},
    archive::{Archive, *},
};

use {
    kutil::std::time::*,
    std::{fs::*, io, path::*},
    tar::*,
};

/// Create a tarball [Archive].
pub fn new_tarball_archive<'own, WriteT>(writer: WriteT) -> ArchiveRef<'own>
where
    WriteT: 'own + io::Write,
{
    Box::new(Builder::new(writer))
}

/// Create a Gzip tarball [Archive].
#[cfg(feature = "gzip")]
pub fn new_gzip_tarball_archive<'own, WriteT>(
    writer: WriteT,
    compression_level: Option<CompressionLevel>,
) -> ArchiveRef<'own>
where
    WriteT: 'own + io::Write,
{
    use flate2::{write::*, *};

    let compression_level =
        compression_level.map(|compression_level| Compression::new(compression_level.to_gzip())).unwrap_or_default();

    let encoder = GzEncoder::new(writer, compression_level);
    new_tarball_archive(encoder)
}

/// Create a Zstandard tarball [Archive].
#[cfg(feature = "zstandard")]
pub fn new_zstandard_tarball_archive<'own, WriteT>(
    writer: WriteT,
    compression_level: Option<CompressionLevel>,
) -> Result<ArchiveRef<'own>, CsarError>
where
    WriteT: 'own + io::Write,
{
    use zstd::stream::*;

    // 0 will choose default (which is 3)
    let compression_level =
        compression_level.map(|compression_level| compression_level.to_zstandard()).unwrap_or_default();

    let encoder = Encoder::new(writer, compression_level)?.auto_finish();
    Ok(new_tarball_archive(encoder))
}

impl<WriteT> Archive for Builder<WriteT>
where
    WriteT: io::Write,
{
    fn add_from_reader(
        &mut self,
        name: &Path,
        mut source: Box<&mut dyn io::Read>,
        size: usize,
        _compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()> {
        let mut header = reader_header(size)?;
        add_from_reader(self, name, &mut header, &mut source, read_tracker)
    }

    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        _compression_level: Option<CompressionLevel>,
        read_tracker: Option<&ReadTrackerRef>,
    ) -> io::Result<()> {
        let mut header = file_header(source)?;
        let mut source = io::BufReader::new(File::open(source)?);
        add_from_reader(self, name, &mut header, &mut source, read_tracker)
    }
}

// Utils

fn add_from_reader<ReadT, WriteT>(
    builder: &mut Builder<WriteT>,
    name: &Path,
    header: &mut Header,
    reader: &mut ReadT,
    read_tracker: Option<&ReadTrackerRef>,
) -> io::Result<()>
where
    ReadT: io::Read,
    WriteT: io::Write,
{
    match read_tracker {
        Some(read_tracker) => builder.append_data(header, name, read_tracker.track(reader)),
        None => builder.append_data(header, name, reader),
    }
}

fn reader_header(size: usize) -> io::Result<Header> {
    let mtime = unix_time().map_err(io::Error::other)?;
    let mut header = Header::new_gnu();
    header.set_mtime(mtime);
    header.set_size(size as u64);
    header.set_mode(DEFAULT_ARCHIVE_ENTRY_PERMISSIONS);
    header.set_cksum();
    Ok(header)
}

fn file_header(path: &Path) -> io::Result<Header> {
    let metadata = path.metadata()?;
    let mut header = Header::new_gnu();
    header.set_metadata(&metadata);
    header.set_cksum();
    Ok(header)
}
