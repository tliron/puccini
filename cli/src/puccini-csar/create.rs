use super::{cli::*, errors::*};

use {
    kutil::{
        cli::{depict::*, run::*},
        std::error::*,
    },
    puccini_csar::*,
    std::{ffi::*, path::*},
};

impl Create {
    /// Run create subcommand.
    pub fn run(&self, cli: &CLI) -> Result<(), MainError> {
        let directory = PathBuf::from(&self.directory);

        let mut csar_errors = Errors::default();

        // Do we already have a TOSCA.meta file?

        let mut meta = Self::get_meta(&directory, &mut csar_errors)?;

        match meta {
            Some(meta) => {
                meta.validate_paths(&directory, &mut csar_errors)?;

                if let Err(csar_errors) = csar_errors.check() {
                    if !cli.quiet {
                        for error in csar_errors {
                            error.print_default_depiction();
                        }
                    }

                    return Err(ExitError::new(1, None).into());
                }
            }

            None => {
                // Create a meta
                let mut new_meta = meta::Meta::default();

                let entry_definitions = Self::find_single_yaml(&directory)?;
                new_meta.entry_definitions = Some(entry_definitions.display().to_string());

                meta = Some(new_meta);
            }
        }

        // Walk directory

        for entry in walkdir::WalkDir::new(&directory).follow_links(true) {
            let entry = entry?;

            if !entry.file_type().is_dir() {
                println!("{}", entry.path().display());
            }
        }

        Ok(())
    }

    fn get_meta(directory: &Path, csar_errors: &mut Errors<CsarError>) -> Result<Option<meta::Meta>, CsarError> {
        Ok(if let Some(meta) = Self::get_meta_path(&directory) {
            Some(meta::Meta::read_path(&meta, csar_errors)?)
        } else {
            None
        })
    }

    fn get_meta_path(directory: &Path) -> Option<PathBuf> {
        let meta = directory.join("TOSCA.meta");
        if meta.exists() {
            return Some(meta);
        } else {
            let meta = directory.join("TOSCA-Metadata").join("TOSCA.meta");
            if meta.exists() {
                return Some(meta);
            }
        }
        None
    }

    fn find_single_yaml(directory: &Path) -> Result<PathBuf, MainError> {
        let mut path = None;

        let extension = Some(OsStr::new("yaml"));

        for entry in walkdir::WalkDir::new(directory).follow_links(true).max_depth(1) {
            let entry = entry?;

            if entry.file_type().is_file() && (entry.path().extension() == extension) {
                if path.is_some() {
                    return Err(ExitError::new(1, Some("multiple `.yaml` files in directory".into())).into());
                }

                path = Some(entry.path().into());
            }
        }

        path.ok_or_else(|| ExitError::new(1, Some("no `.yaml` file in directory".into())).into())
    }
}
