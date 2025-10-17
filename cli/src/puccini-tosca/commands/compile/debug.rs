use clap::*;

//
// Debug
//

/// Debug flag.
#[derive(Clone, ValueEnum)]
pub enum Debug {
    /// output namespaces (stops before completion phase)
    Namespaces,

    /// output parsed entities (stops before completion phase)
    Parsed,

    /// output completed entities (stops before compilation phase)
    Completed,

    /// output compiled Floria templates (even if there are errors)
    Compiled,

    /// output instantiated Floria entities (even if there are errors)
    Instance,
}

impl ToString for Debug {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}
