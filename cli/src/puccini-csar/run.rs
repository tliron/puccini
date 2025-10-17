use super::{commands::*, errors::*};

use {clap::*, kutil::cli::log::*};

/// Run.
pub fn run() -> Result<(), MainError> {
    let root = Root::parse();

    if !root.quiet {
        root.colorize.initialize();
        initialize_tracing(root.verbose + 2, root.log_path.as_ref())?;
    }

    match &root.subcommand {
        None => {}
        Some(subcommand) => match subcommand {
            SubCommand::Create(create) => create.run(&root)?,
            SubCommand::Meta(meta) => meta.run(&root)?,
            SubCommand::Inspect(inspect) => inspect.run(&root)?,
            SubCommand::Version(version) => version.run::<Root>(),
            SubCommand::Completion(completion) => completion.run::<Root>(),
            SubCommand::Manual(manual) => manual.run::<Root>()?,
        },
    }

    Ok(())
}
