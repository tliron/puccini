use super::super::super::entities::*;

use {floria_plugin_sdk::data::*, std::iter::*};

impl<'context> ToscaPathParser<'context> {
    /// Next selector for node or relationship.
    pub fn next_selector(&mut self) -> Result<ToscaSelector, String> {
        Ok(match self.iterator.peek() {
            Some(index) => match index {
                // ALL
                Expression::Text(text) => match text.as_str() {
                    "ALL" => {
                        self.iterator.next();
                        ToscaSelector::All
                    }

                    _ => Default::default(),
                },

                // <integer_index>
                Expression::Integer(integer) => {
                    self.iterator.next();
                    ToscaSelector::Index(*integer as usize)
                }

                // <integer_index>
                Expression::UnsignedInteger(unsigned_integer) => {
                    self.iterator.next();
                    ToscaSelector::Index(*unsigned_integer as usize)
                }

                _ => Default::default(),
            },

            None => Default::default(),
        })
    }
}
