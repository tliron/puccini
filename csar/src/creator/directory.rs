use super::{
    super::{errors::*, meta::*},
    archive::*,
    creator::*,
    format::*,
};

use {kutil::std::error::*, std::path::*};

impl CsarCreator {
    /// Create CSAR file from directory.
    ///
    /// If file is [None] will output to [io::stdout].
    ///
    /// If the format is not provided it will be selected according to the archive extension or default
    /// to [Format::Tarball].
    ///
    /// Note that for [Format::ZIP] if file is [None] will return an error.
    pub fn create_from_directory<ErrorRecipientT>(
        mut self,
        file: Option<&Path>,
        directory: &Path,
        dry_run: bool,
        errors: &mut ErrorRecipientT,
    ) -> Result<(Option<Meta>, Option<u64>), CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        if !directory.is_dir() {
            errors.give(CsarError::Invalid(format!("not a directory: {:?}", directory.display())))?;
            return Ok((None, None));
        }

        // Do we already have a TOSCA.meta file?

        let mut meta = match Meta::from_directory(directory, errors)? {
            Some(meta) => {
                tracing::info!("using existing \"TOSCA.meta\"");
                meta.complete()
            }

            None => {
                tracing::info!("creating \"TOSCA.meta\"");
                self.into_meta()
            }
        };

        meta.validate_definitions_in_directory(directory, errors)?;

        // Do we already have entry_definitions?

        if meta.entry_definitions.is_none() {
            let entry_definitions =
                unwrap_or_give_and_return!(entry_definitions_in_directory(directory), errors, Ok((None, None)));

            match entry_definitions.to_str() {
                Some(entry_definitions) => meta.entry_definitions = Some(entry_definitions.into()),
                None => {
                    errors.give(CsarError::Invalid(format!("path not UTF-8: {}", entry_definitions.display())))?;
                    return Ok((None, None));
                }
            }
        }

        let meta_string = meta.stringify(self.max_columns)?;

        // Determine format

        let format = match self.format {
            Some(format) => format,

            None => match file {
                Some(file) => match Format::from_extension(file) {
                    Some(format) => format,

                    None => {
                        errors.give(CsarError::Invalid("must specify format".into()))?;
                        return Ok((None, None));
                    }
                },

                None => Format::Tarball, // for stdout
            },
        };

        // Create archive

        let mut archive: Option<ArchiveRef> = if dry_run {
            None
        } else {
            Some(unwrap_or_give_and_return!(
                create_archive_file_or_stdout(file, format, self.compression),
                errors,
                Ok((None, None))
            ))
        };

        let directory_components = directory.components().count();
        let locations = meta_locations();

        // Create progress bar

        let (progress_bar, size) = match self.progress_bar {
            Some(progress_bar) => {
                let size = directory_size(directory, directory_components, &locations, errors)?;
                progress_bar.set_length(size);
                (Some(progress_bar), Some(size))
            }

            None => (None, None),
        };

        // Add TOSCA.meta to CSAR
        // (It's important for tarballs that it's the first file added!)

        if let Some(archive) = &mut archive {
            tracing::debug!("adding: \"TOSCA.meta\"");
            if let Err(error) = archive.add_string(
                locations.get(0).expect("not empty"),
                &meta_string,
                self.compression,
                progress_bar.as_ref(),
            ) {
                errors.give(error)?;
            }
        }

        // Add directory to CSAR

        for entry in files_in_directory(&directory, true) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_dir() {
                        continue;
                    }

                    let path = entry.path();
                    let name = path_to_name(path, directory_components);

                    if locations.contains(&name) {
                        tracing::debug!("skipping: {:?}", path.display());
                        continue;
                    }

                    tracing::debug!("adding: {:?}", path.display());

                    if let Some(archive) = &mut archive
                        && let Err(error) = archive.add_file(name, path, self.compression, progress_bar.as_ref())
                    {
                        errors.give(error)?;
                    }
                }

                Err(error) => errors.give(error)?,
            }
        }

        Ok((Some(meta), size))
    }
}

// Utils

fn directory_size<ErrorRecipientT>(
    directory: &Path,
    directory_components: usize,
    locations: &Vec<PathBuf>,
    errors: &mut ErrorRecipientT,
) -> Result<u64, CsarError>
where
    ErrorRecipientT: ErrorRecipient<CsarError>,
{
    let mut size = 0;

    for entry in files_in_directory(&directory, false) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    continue;
                }

                let name = path_to_name(entry.path(), directory_components);
                if locations.contains(&name) {
                    continue;
                }

                match entry.metadata() {
                    Ok(metadata) => size += metadata.len(),
                    Err(error) => errors.give(error)?,
                }
            }

            Err(error) => errors.give(error)?,
        }
    }

    Ok(size)
}

fn files_in_directory(directory: &Path, sort: bool) -> walkdir::WalkDir {
    let mut walkdir = walkdir::WalkDir::new(&directory).follow_links(true);
    if sort {
        walkdir = walkdir.sort_by_file_name();
    }
    walkdir
}

fn path_to_name(path: &Path, count: usize) -> PathBuf {
    path.components().skip(count).collect()
}
