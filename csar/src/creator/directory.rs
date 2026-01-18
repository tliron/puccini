use super::{
    super::{errors::*, tosca_meta::*},
    archive::*,
    created::*,
    creator::*,
    format::*,
};

use {
    problemo::{common::*, *},
    std::{fs::*, io::Write, path::*},
};

impl CsarCreator {
    /// Create CSAR file from directory.
    ///
    /// If file is [None] will output to [stdout](std::io::stdout).
    ///
    /// If the format is not provided it will be selected according to the archive extension or default
    /// to [Format::Tarball].
    ///
    /// Note that for [Format::ZIP] if file is [None] will return an error.
    pub fn create_from_directory<ProblemReceiverT>(
        &self,
        file: Option<&Path>,
        directory: &Path,
        write_tosca_meta: bool,
        delete_tosca_meta: bool,
        dry_run: bool,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<CreatedCsar>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        if !directory.is_dir() {
            problems
                .give(InvalidError::as_problem(format!("not a directory: {:?}", directory.display())).via(CsarError))?;
            return Ok(None);
        }

        let locations = tosca_meta_locations();

        if delete_tosca_meta {
            for location in &locations {
                let location = directory.join(location);
                if location.exists() {
                    give_unwrap!(remove_file(location), problems);
                }
            }
        }

        // Do we already have a TOSCA.meta file?

        let mut tosca_meta = match ToscaMeta::from_directory(directory, problems)? {
            Some(tosca_meta) => {
                tracing::info!("using existing \"TOSCA.meta\"");
                tosca_meta.complete()
            }

            None => {
                tracing::info!("creating \"TOSCA.meta\"");
                self.to_tosca_meta()
            }
        };

        tosca_meta.validate_definitions_in_directory(directory, problems)?;

        let directory_components = directory.components().count();

        // Do we already have entry_definitions?

        if tosca_meta.entry_definitions.is_none() {
            let entry_definitions =
                path_to_name(&give_unwrap!(entry_definitions_in_directory(directory), problems), directory_components);

            match entry_definitions.to_str() {
                Some(entry_definitions) => tosca_meta.entry_definitions = Some(entry_definitions.into()),
                None => {
                    problems.give(
                        InvalidError::as_problem(format!("path not UTF-8: {}", entry_definitions.display()))
                            .via(CsarError),
                    )?;
                    return Ok(None);
                }
            }
        }

        let tosca_meta_string = tosca_meta.stringify(self.max_columns)?;

        if write_tosca_meta {
            let location = directory.join(locations.get(0).expect("not empty"));
            let mut file = give_unwrap!(File::create_new(location), problems);
            give_unwrap!(file.write_all(tosca_meta_string.as_bytes()), problems);
            return Ok(Some(CreatedCsar::new(tosca_meta, Format::Tarball, None, None)));
        }

        // Determine format

        let format = match self.format {
            Some(format) => format,

            None => match file {
                Some(file) => match Format::from_extension(file) {
                    Some(format) => format,

                    None => {
                        problems.give(InvalidError::as_problem("must specify format").via(CsarError))?;
                        return Ok(None);
                    }
                },

                None => Format::Tarball, // for stdout
            },
        };

        // Create archive

        let mut archive: Option<ArchiveWriterRef> = if dry_run {
            None
        } else {
            Some(give_unwrap!(create_archive_file_or_stdout(file, format, self.compression_level), problems))
        };

        // Initialize read tracker and size

        let (read_tracker, size) = match &self.read_tracker {
            Some(read_tracker) => {
                let size = directory_size(directory, directory_components, &locations, problems)?;
                read_tracker.initialize(size);
                (Some(read_tracker), Some(size))
            }

            None => (None, None),
        };

        // Add TOSCA.meta to CSAR
        // (It's important for tarballs that it's the first file added!)

        if let Some(archive) = &mut archive {
            tracing::debug!("adding: \"TOSCA.meta\"");
            give_unwrap!(
                archive.add_string(
                    locations.get(0).expect("not empty"),
                    &tosca_meta_string,
                    self.compression_level,
                    read_tracker,
                ),
                problems
            );
        }

        // Add directory to CSAR

        for entry in files_in_directory(&directory, true) {
            if let Some(entry) = entry.give_ok(problems)? {
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
                    give_unwrap!(archive.add_file(name, path, self.compression_level, read_tracker), problems);
                }
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

fn directory_size<ProblemReceiverT>(
    directory: &Path,
    directory_components: usize,
    locations: &Vec<PathBuf>,
    problems: &mut ProblemReceiverT,
) -> Result<u64, Problem>
where
    ProblemReceiverT: ProblemReceiver,
{
    let mut size = 0;

    for entry in files_in_directory(&directory, false) {
        if let Some(entry) = entry.give_ok(problems)? {
            if entry.file_type().is_dir() {
                continue;
            }

            let name = path_to_name(entry.path(), directory_components);
            if locations.contains(&name) {
                continue;
            }

            if let Some(metadata) = entry.metadata().give_ok(problems)? {
                size += metadata.len();
            }
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
