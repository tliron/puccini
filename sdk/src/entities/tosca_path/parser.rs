use {
    floria_plugin_sdk::data::*,
    std::{iter::*, slice::*},
};

//
// ToscaPathParser
//

/// TOSCA Path parser.
pub struct ToscaPathParser<'context> {
    /// Iterator.
    pub iterator: Peekable<Iter<'context, Expression>>,
}

impl<'context> ToscaPathParser<'context> {
    /// Constructor.
    pub fn new(arguments: &'context Vec<Expression>) -> Self {
        Self { iterator: arguments.iter().peekable() }
    }
}
