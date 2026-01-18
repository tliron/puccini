use super::command::*;

use {anstream::eprintln, depiction::*, problemo::*, puccini_csar::creator::*};

impl Meta {
    pub fn print_introduction(&self) {
        let prefix = self.prefix();

        eprintln!("{}Reading from directory {:?}", prefix, DEFAULT_THEME.string(self.directory.display()));
    }

    pub fn print_summary(&self, created: CreatedCsar) -> Result<(), Problem> {
        let prefix = self.prefix();

        eprintln!("{}Wrote {:?}", prefix, DEFAULT_THEME.string(self.directory.join("TOSCA.meta").display()));

        eprintln!();
        created.tosca_meta.eprint_default_depiction();

        Ok(())
    }

    fn prefix(&self) -> String {
        if self.dry_run { format!("{}", DEFAULT_THEME.meta("(dry run) ")) } else { Default::default() }
    }
}
