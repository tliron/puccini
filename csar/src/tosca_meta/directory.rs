use super::{super::errors::*, tosca_meta::*};

use {
    kutil::std::error::*,
    std::{ffi::*, path::*},
    walkdir::WalkDir,
};

impl ToscaMeta {
    /// From directory.
    pub fn from_directory<ErrorReceiverT>(
        directory: &Path,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<ToscaMeta>, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        Ok(if let Some(path) = unwrap_or_give_and_return!(tosca_meta_path_in_directory(directory), errors, Ok(None)) {
            Some(ToscaMeta::read_path(&path, errors)?)
        } else {
            None
        })
    }

    /// Validate definitions in directory.
    pub fn validate_definitions_in_directory<ErrorReceiverT>(
        &self,
        directory: &Path,
        errors: &mut ErrorReceiverT,
    ) -> Result<(), CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        // Note that .exists() follows symbolic links, which is what we want

        if let Some(entry_definitions) = &self.entry_definitions {
            let path = directory.join(entry_definitions);
            if !path.exists() {
                errors.give(ToscaMetaError::FileNotFound(path.display().to_string()))?;
            }
        }

        for other_definition in &self.other_definitions {
            let path = directory.join(other_definition);
            if !path.exists() {
                errors.give(ToscaMetaError::FileNotFound(path.display().to_string()))?;
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
pub fn tosca_meta_path_in_directory(directory: &Path) -> Result<Option<PathBuf>, CsarError> {
    let mut path = None;

    for location in tosca_meta_locations() {
        let location = directory.join(location);
        if location.exists() {
            if path.is_some() {
                return Err(CsarError::Invalid("multiple \"TOSCA.meta\" files in directory".into()));
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
pub fn entry_definitions_in_directory(directory: &Path) -> Result<PathBuf, CsarError> {
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
                    return Err(CsarError::Invalid(
                        "can't generate \"Entry-Definitions\": multiple YAML files in directory".into(),
                    ));
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

        None => Err(CsarError::Invalid("can't generate \"Entry-Definitions\": no YAML files in directory".into())),
    }
}
