use super::command::*;

use {
    anstream::{eprint, eprintln},
    depiction::*,
    indicatif::*,
    problemo::*,
    puccini_csar::creator::*,
};

impl Create {
    pub fn print_introduction(&self) {
        let prefix = self.prefix();

        eprintln!("{}Reading from directory {:?}", prefix, DEFAULT_THEME.string(self.directory.display()));

        let output = match &self.file {
            Some(archive_path) => format!("{:?}", DEFAULT_THEME.string(archive_path.display())),
            None => format!("{}", DEFAULT_THEME.name("stdout")),
        };

        eprintln!("{}Writing to {}", prefix, output);
    }

    pub fn print_summary(&self, created: CreatedCsar) -> Result<(), Problem> {
        let prefix = self.prefix();

        if let Some(compressed) = created.size {
            let compression_level = match created.compression_level {
                Some(compression_level) => format!("{}", DEFAULT_THEME.number(compression_level.to_string())),
                None => format!("{}", DEFAULT_THEME.symbol("default")),
            };

            eprintln!(
                "{}Compressed {} (level = {})",
                prefix,
                DEFAULT_THEME.number(HumanBytes(compressed)),
                compression_level
            );
        }

        eprint!("{}Wrote {}", prefix, DEFAULT_THEME.name(created.format));

        if let Some(file) = &self.file {
            let size = file.metadata()?.len();
            eprint!(" {}", DEFAULT_THEME.number(HumanBytes(size)));

            if let Some(compressed) = created.size {
                eprint!(" ({:.0}%)", 100. * (size as f64) / (compressed as f64));
            }
        }

        eprintln!();
        eprintln!();

        created.tosca_meta.eprint_default_depiction();

        Ok(())
    }

    fn prefix(&self) -> String {
        if self.dry_run { format!("{}", DEFAULT_THEME.meta("(dry run) ")) } else { Default::default() }
    }
}
