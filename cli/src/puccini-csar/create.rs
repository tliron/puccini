use puccini_csar::creator::CompressionLevel;

use super::{cli::*, errors::*};

use {
    anstream::eprintln,
    indicatif::*,
    kutil::{
        cli::{depict::*, run::*},
        std::error::*,
    },
    puccini_csar::{Meta, *},
};

impl Create {
    /// Run create subcommand.
    pub fn run(&self, cli: &CLI) -> Result<(), MainError> {
        let mut csar_errors = Errors::default();

        if !cli.quiet {
            self.print_introduction();
        }

        let (meta, compressed) = self.creator(cli).create_from_directory(
            self.file.as_ref().map(|path| path.as_ref()),
            &self.directory,
            self.dry_run,
            &mut csar_errors,
        )?;

        match csar_errors.check() {
            Ok(()) => {
                if !cli.quiet {
                    self.print_summary(meta, compressed)?;
                }

                Ok(())
            }

            Err(csar_errors) => {
                if !cli.quiet {
                    for error in csar_errors {
                        error.eprint_default_depiction();
                    }
                }

                Err(ExitError::new(1, None).into())
            }
        }
    }

    fn print_introduction(&self) {
        let description = match &self.file {
            Some(archive_path) => format!("{:?}", archive_path.display()),
            None => "stdout".into(),
        };

        if self.dry_run {
            eprintln!("{} Writing CSAR to {}", DEFAULT_THEME.meta("(dry run)"), DEFAULT_THEME.string(description));
            eprintln!(
                "{} Adding directory {:?}",
                DEFAULT_THEME.meta("(dry run)"),
                DEFAULT_THEME.string(self.directory.display())
            );
        } else {
            eprintln!("Writing CSAR to {}", DEFAULT_THEME.string(description));
            eprintln!("Adding directory {:?}", DEFAULT_THEME.string(self.directory.display()));
        }
    }

    fn print_summary(&self, meta: Option<Meta>, compressed: Option<u64>) -> Result<(), MainError> {
        if self.dry_run {
            eprintln!("{} Created CSAR", DEFAULT_THEME.meta("(dry run)"));
        } else {
            if let Some(compressed) = compressed {
                eprintln!("Compressed: {}", DEFAULT_THEME.number(indicatif::HumanBytes(compressed)));
            }

            if let Some(file) = &self.file {
                let size = file.metadata()?.len();
                if let Some(compressed) = compressed {
                    eprintln!(
                        "Created CSAR: {} ({:.0}%)",
                        DEFAULT_THEME.number(indicatif::HumanBytes(size)),
                        100. * (size as f64) / (compressed as f64)
                    );
                } else {
                    eprintln!("Created CSAR: {}", DEFAULT_THEME.number(indicatif::HumanBytes(size)));
                }
            }
        }

        if let Some(meta) = meta {
            eprintln!();
            meta.eprint_default_depiction();
        }

        Ok(())
    }

    fn creator(&self, cli: &CLI) -> creator::CsarCreator {
        creator::CsarCreator::new(
            self.csar_format(),
            CompressionLevel::new_unchecked(self.compression_level),
            self.created_by.clone(),
            self.entry_definitions.clone(),
            self.other_definitions.clone(),
            Some(self.max_columns),
            if cli.quiet { None } else { Some(Self::progress_bar(cli)) },
            cli.colorize.colorize(),
        )
    }

    fn csar_format(&self) -> Option<creator::Format> {
        self.format.as_ref().map(|format| format.to_puccini())
    }

    fn progress_bar(cli: &CLI) -> ProgressBar {
        let progress_bar = ProgressBar::no_length();
        progress_bar.set_prefix("Compressing");
        progress_bar.set_style(
            ProgressStyle::with_template(if cli.colorize.colorize() {
                "{prefix} {binary_total_bytes:.magenta} {bar:40.green/white} {spinner:.yellow} {bytes_per_sec} ~{eta} left"
            } else {
                "{prefix} {binary_total_bytes} {bar:40} {spinner} {bytes_per_sec} ~{eta} left"
            })
            .expect("ProgressStyle::with_template")
            .progress_chars("█▓▒")
            .tick_chars("▁▂▃▄▅▆▇█▇▆▅▄▃▂▁"),
        );
        progress_bar
    }
}
