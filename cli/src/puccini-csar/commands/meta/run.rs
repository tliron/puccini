use super::{super::root::*, command::*};

use {
    compris::depict::*,
    depiction::*,
    problemo::{common::*, *},
    puccini_csar::*,
};

impl Meta {
    /// Run meta subcommand.
    pub fn run(&self, root: &Root) -> Result<(), Problem> {
        let mut problems = Problems::default();

        if !root.quiet {
            self.print_introduction();
        }

        let created = self.creator().create_from_directory(
            None,
            &self.directory,
            true,
            self.force,
            self.dry_run,
            &mut problems,
        )?;

        match problems.check() {
            Ok(_) => {
                if !root.quiet
                    && let Some(created) = created
                {
                    self.print_summary(created)?;
                }

                Ok(())
            }

            Err(problems) => {
                if !root.quiet {
                    problems.annotated_depiction().eprint_default_depiction();
                }

                Err(ExitError::failure())
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
