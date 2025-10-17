use super::{cli::*, errors::*};

use {clap::*, kutil::cli::log::*};

pub fn run() -> Result<(), MainError> {
    let cli = CLI::parse();

    if !cli.quiet {
        cli.colorize.initialize();
        initialize_tracing(cli.verbose + 2, cli.log_path.as_ref())?;
    }

    match &cli.subcommand {
        None => {}
        Some(subcommand) => match subcommand {
            SubCommand::Create(create) => create.run(&cli)?,
            SubCommand::Extract(extract) => extract.run(&cli)?,
            SubCommand::Meta(meta) => meta.run(&cli)?,
            SubCommand::Version(version) => version.run::<CLI>(),
            SubCommand::Completion(completion) => completion.run::<CLI>(),
            SubCommand::Manual(manual) => manual.run::<CLI>()?,
        },
    }

    Ok(())
}
