use super::{
    super::{errors::*, tosca_meta::*},
    archive::*,
    created::*,
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
    ) -> Result<Option<CreatedCsar>, CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        if !directory.is_dir() {
            errors.give(CsarError::Invalid(format!("not a directory: {:?}", directory.display())))?;
            return Ok(None);
        }

        // Do we already have a TOSCA.meta file?

        let mut tosca_meta = match ToscaMeta::from_directory(directory, errors)? {
            Some(tosca_meta) => {
                tracing::info!("using existing \"TOSCA.meta\"");
                tosca_meta.complete()
            }

            None => {
                tracing::info!("creating \"TOSCA.meta\"");
                self.into_tosca_meta()
            }
        };

        tosca_meta.validate_definitions_in_directory(directory, errors)?;

        // Do we already have entry_definitions?

        if tosca_meta.entry_definitions.is_none() {
            let entry_definitions =
                unwrap_or_give_and_return!(entry_definitions_in_directory(directory), errors, Ok(None));

            match entry_definitions.to_str() {
                Some(entry_definitions) => tosca_meta.entry_definitions = Some(entry_definitions.into()),
                None => {
                    errors.give(CsarError::Invalid(format!("path not UTF-8: {}", entry_definitions.display())))?;
                    return Ok(None);
                }
            }
        }

        let tosca_meta_string = tosca_meta.stringify(self.max_columns)?;

        // Determine format

        let format = match self.format {
            Some(format) => format,

            None => match file {
                Some(file) => match Format::from_extension(file) {
                    Some(format) => format,

                    None => {
                        errors.give(CsarError::Invalid("must specify format".into()))?;
                        return Ok(None);
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
                create_archive_file_or_stdout(file, format, self.compression_level),
                errors,
                Ok(None)
            ))
        };

        let directory_components = directory.components().count();
        let locations = tosca_meta_locations();

        // Initialize read tracker and size

        let (read_tracker, size) = match &self.read_tracker {
            Some(read_tracker) => {
                let size = directory_size(directory, directory_components, &locations, errors)?;
                read_tracker.initialize(size);
                (Some(read_tracker), Some(size))
            }

            None => (None, None),
        };

        // Add TOSCA.meta to CSAR
        // (It's important for tarballs that it's the first file added!)

        if let Some(archive) = &mut archive {
            tracing::debug!("adding: \"TOSCA.meta\"");
            unwrap_or_give!(
                archive.add_string(
                    locations.get(0).expect("not empty"),
                    &tosca_meta_string,
                    self.compression_level,
                    read_tracker,
                ),
                errors
            );
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

                    if let Some(archive) = &mut archive {
                        unwrap_or_give!(archive.add_file(name, path, self.compression_level, read_tracker), errors);
                    }
                }

                Err(error) => errors.give(error)?,
            }
        }

        if let Some(read_tracker) = read_tracker {
            tracing::info!("finishing read tracker");
            read_tracker.finish(true);
        }

        Ok(Some(CreatedCsar::new(tosca_meta, format, self.compression_level, size)))
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
