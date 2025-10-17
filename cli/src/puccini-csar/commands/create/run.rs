use super::{
    super::{super::errors::*, root::*},
    command::*,
};

use {
depiction::*,    kutil::{
        cli::{ run::*},
        std::error::*,
    },
    puccini_csar::{creator::*, *},
};

impl Create {
    /// Run create subcommand.
    pub fn run(&self, root: &Root) -> Result<(), MainError> {
        let mut csar_errors = Errors::default();

        if !root.quiet {
            self.print_introduction();
        }

        let created = self.creator(root).create_from_directory(
            self.file.as_ref().map(|path| path.as_ref()),
            &self.directory,
            false,
            false,
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

    fn creator(&self, root: &Root) -> creator::CsarCreator {
        creator::CsarCreator::new(
            self.csar_format(),
            self.compression_level(),
            self.created_by.clone(),
            self.entry_definitions.clone(),
            self.other_definitions.clone(),
            Some(self.max_columns),
            if root.quiet {
                None
            } else {
                Some(Box::new(ReadTrackerChain::new(vec![
                    Self::con_emu_osc_progress_state(),
                    // indicatif second so it can clean up the line before OSC code is sent
                    Self::indicatif_progress_bar(root),
                ])))
            },
        )
    }
}
