use super::{
    super::{super::errors::*, root::*},
    command::*,
};

use {
    depiction::*,
    kutil::{cli::run::*, std::error::*},
    puccini_csar::*,
};

impl Meta {
    /// Run meta subcommand.
    pub fn run(&self, root: &Root) -> Result<(), MainError> {
        let mut csar_errors = Errors::default();

        if !root.quiet {
            self.print_introduction();
        }

        let created = self.creator().create_from_directory(
            None,
            &self.directory,
            true,
            self.force,
            self.dry_run,
            &mut csar_errors,
        )?;

        match csar_errors.check() {
            Ok(()) => {
                if !root.quiet
                    && let Some(created) = created
                {
                    self.print_summary(created)?;
                }

                Ok(())
            }

            Err(csar_errors) => {
                if !root.quiet {
                    for error in csar_errors {
                        error.eprint_default_depiction();
                    }
                }

                Err(ExitError::new(1, None).into())
            }
        }
    }

    fn creator(&self) -> creator::CsarCreator {
        creator::CsarCreator::new(
            None,
            None,
            self.created_by.clone(),
            self.entry_definitions.clone(),
            self.other_definitions.clone(),
            Some(self.max_columns),
            None,
        )
    }
}
