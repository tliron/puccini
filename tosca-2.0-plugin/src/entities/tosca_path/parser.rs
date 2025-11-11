use {
    floria_plugin_sdk::data::*,
    std::{iter::*, slice::*},
};

//
// ToscaPathParser
//

/// TOSCA Path parser.
pub struct ToscaPathParser<'own> {
    /// Iterator.
    pub iterator: Peekable<Iter<'own, Expression>>,
}

impl<'own> ToscaPathParser<'own> {
    /// Constructor.
    pub fn new(arguments: &'own Vec<Expression>) -> Self {
        Self { iterator: arguments.iter().peekable() }
    }
}
