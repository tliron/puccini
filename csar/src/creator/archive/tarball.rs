use super::{
    super::{super::errors::*, compression_level::*},
    archive::{Archive, *},
};

use {
    flate2::{write::*, *},
    indicatif::*,
    kutil::std::time::*,
    std::{fs::*, io, path::*},
    tar::*,
};

/// Create an [Archive] for a tarball file or write to stdout.
pub fn create_tarball_file_or_stdout(path: Option<&Path>) -> Result<ArchiveRef, CsarError> {
    let writer: Box<dyn io::Write> = match path {
        Some(file) => {
            tracing::info!("writing tarball to {}", file.display());
            Box::new(File::create(file)?)
        }

        None => {
            tracing::info!("writing tarball to stdout");
            Box::new(io::stdout())
        }
    };

    let tar_builder = Builder::new(writer);
    Ok(Box::new(tar_builder))
}

/// Create an [Archive] for a Gzip tarball file or write to stdout.
pub fn create_gzip_tarball_file_or_stdout(
    path: Option<&Path>,
    compression: CompressionLevel,
) -> Result<ArchiveRef, CsarError> {
    let writer: Box<dyn io::Write> = match path {
        Some(file) => {
            tracing::info!("writing Gzip tarball to {}", file.display());
            Box::new(File::create(file)?)
        }

        None => {
            tracing::info!("writing Gzip tarball to stdout");
            Box::new(io::stdout())
        }
    };

    let gz_encoder = GzEncoder::new(writer, Compression::new(compression.to_gzip()));
    let tar_builder = Builder::new(gz_encoder);
    Ok(Box::new(tar_builder))
}

impl<WriteT> Archive for Builder<WriteT>
where
    WriteT: io::Write,
{
    fn add_from_reader(
        &mut self,
        name: &Path,
        source: Box<&mut dyn io::Read>,
        size: usize,
        _compression: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()> {
        let mut header = reader_header(size)?;

        match progress_bar {
            Some(progress_bar) => self.append_data(&mut header, name, progress_bar.wrap_read(source)),
            None => self.append_data(&mut header, name, source),
        }
    }

    fn add_from_file(
        &mut self,
        name: &Path,
        source: &Path,
        _compression: CompressionLevel,
        progress_bar: Option<&ProgressBar>,
    ) -> io::Result<()> {
        let mut header = file_header(source)?;
        let file = File::open(source)?;

        match progress_bar {
            Some(progress_bar) => self.append_data(&mut header, name, progress_bar.wrap_read(file)),
            None => self.append_data(&mut header, name, file),
        }
    }
}

// Utils

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
