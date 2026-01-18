use super::{super::errors::*, tosca_meta::*};

use {
    problemo::{common::*, *},
    std::{ffi::*, path::*},
    walkdir::WalkDir,
};

impl ToscaMeta {
    /// From directory.
    pub fn from_directory<ProblemReceiverT>(
        directory: &Path,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<ToscaMeta>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(if let Some(path) = give_unwrap!(tosca_meta_path_in_directory(directory), problems) {
            Some(ToscaMeta::read_path(&path, problems)?)
        } else {
            None
        })
    }

    /// Validate definitions in directory.
    pub fn validate_definitions_in_directory<ProblemReceiverT>(
        &self,
        directory: &Path,
        problems: &mut ProblemReceiverT,
    ) -> Result<(), Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        // Note that .exists() follows symbolic links, which is what we want

        if let Some(entry_definitions) = &self.entry_definitions {
            let path = directory.join(entry_definitions);
            if !path.exists() {
                problems.give(NotFoundError::default_as_problem().with(path).via(CsarError))?;
            }
        }

        for other_definition in &self.other_definitions {
            let path = directory.join(other_definition);
            if !path.exists() {
                problems.give(NotFoundError::default_as_problem().with(path).via(CsarError))?;
            }
        }

        Ok(())
    }
}

/// Supported `TOSCA.meta` locations, ordered by priority.
pub fn tosca_meta_locations() -> Vec<PathBuf> {
    vec![PathBuf::from("TOSCA.meta"), PathBuf::from("TOSCA-Metadata").join("TOSCA.meta")]
}

/// Find `TOSCA.meta` file in directory.
///
/// Will test all locations from [tosca_meta_locations].
///
/// If more than one location is used then an error is returned.
pub fn tosca_meta_path_in_directory(directory: &Path) -> Result<Option<PathBuf>, Problem> {
    let mut path = None;

    for location in tosca_meta_locations() {
        let location = directory.join(location);
        if location.exists() {
            if path.is_some() {
                return Err(InvalidError::as_problem("multiple \"TOSCA.meta\" files in directory").via(CsarError));
            }

            path = Some(location);
        }
    }

    Ok(path)
}

/// Find entry definitions in directory.
///
/// It must be a single `.yaml` or `.yml` file in the directory root.
///
/// If more than one suitable file is found then an error is returned.
pub fn entry_definitions_in_directory(directory: &Path) -> Result<PathBuf, Problem> {
    let mut entry_definitions: Option<PathBuf> = None;

    let extensions = vec![OsStr::new("yaml"), OsStr::new("yml")];

    for entry in WalkDir::new(directory).follow_links(true).max_depth(1) {
        let entry = entry?;

        if entry.file_type().is_file() {
            let path = entry.path();

            if let Some(extension) = path.extension()
                && extensions.contains(&extension)
            {
                tracing::debug!("\"Entry-Definitions\" candidate: {:?}", path.display());

                if entry_definitions.is_some() {
                    return Err(InvalidError::as_problem(
                        "can't generate \"Entry-Definitions\": multiple YAML files in directory",
                    )
                    .via(CsarError));
                }

                entry_definitions = Some(path.into());
            }
        }
    }

    match entry_definitions {
        Some(entry_definitions) => {
            tracing::info!("found \"Entry-Definitions\": {:?}", entry_definitions.display());
            Ok(entry_definitions)
        }

        None => {
            Err(InvalidError::as_problem("can't generate \"Entry-Definitions\": no YAML files in directory")
                .via(CsarError))
        }
    }
}
