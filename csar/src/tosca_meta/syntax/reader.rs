use super::super::{super::errors::*, block::*};

use {
    std::{io, iter::*},
    utf8_chars::*,
};

//
// ToscaMetaBlockReader
//

/// [ToscaMetaBlock] reader.
///
/// A TOSCA.meta file consists of keyname/value pairs. The keyname of a keyname/value pair is
/// followed by a colon, followed by a space, followed by the value of the keyname/value pair. The
/// keyname MUST NOT contain a colon. Values that represent binary data MUST be base64 encoded.
/// Values that extend beyond one line can be spread over multiple lines if each subsequent line
/// starts with at least one space. Such spaces are then collapsed when the value string is read.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Debug)]
pub struct ToscaMetaBlockReader<'read, ReadT>
where
    ReadT: io::BufRead,
{
    /// Characters.
    pub chars: Peekable<Chars<'read, ReadT>>,
}

impl<'read, ReadT> ToscaMetaBlockReader<'read, ReadT>
where
    ReadT: io::BufRead,
{
    /// Constructor.
    pub fn new(chars: Peekable<Chars<'read, ReadT>>) -> Self {
        Self { chars }
    }

    /// Constructor.
    pub fn new_from(read: &'read mut ReadT) -> Self
    where
        ReadT: io::BufRead,
    {
        Self::new(read.chars().peekable())
    }

    /// True if there are no more characters to read.
    pub fn is_empty(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    /// Read block.
    pub fn read_block(&mut self) -> Result<Option<ToscaMetaBlock>, CsarError> {
        match self.chars.peek() {
            Some(c) => match c {
                Ok(c) => {
                    if *c == '\n' {
                        self.skip()?;
                        Ok(None)
                    } else {
                        let mut block = ToscaMetaBlock::default();
                        while let Some((keyname, value)) = self.read_key()? {
                            block.insert(keyname, value)?;
                        }
                        Ok(Some(block))
                    }
                }

                Err(error) => Err(ToscaMetaError::Malformed(error.to_string()).into()),
            },

            None => Ok(None),
        }
    }

    /// Read key.
    pub fn read_key(&mut self) -> Result<Option<(String, String)>, CsarError> {
        let mut mode = KeyValueMode::Key;

        let mut keyname = String::default();
        let mut value = String::default();

        while let Some(next) = self.chars.next() {
            let next = next?;

            match next {
                '\n' => {
                    if !self.continues()? {
                        break;
                    }
                }

                ':' => match mode {
                    KeyValueMode::Key => {
                        self.must_skip_space()?;
                        mode = KeyValueMode::Value;
                    }

                    KeyValueMode::Value => {}
                },

                _ => match mode {
                    KeyValueMode::Key => keyname.push(next),
                    KeyValueMode::Value => value.push(next),
                },
            }
        }

        if !keyname.is_empty() {
            if matches!(mode, KeyValueMode::Key) {
                return Err(ToscaMetaError::Malformed("keyname not followed by `:`".into()).into());
            }

            Ok(Some((keyname, value)))
        } else {
            Ok(None)
        }
    }

    fn skip(&mut self) -> io::Result<()> {
        if let Some(next) = self.chars.next() {
            next?;
        }
        Ok(())
    }

    fn must_skip_space(&mut self) -> Result<(), CsarError> {
        if let Some(next) = self.chars.next() {
            let next = next?;
            if next != ' ' {
                return Err(ToscaMetaError::Malformed("`:` not followed by space".into()).into());
            }
        }
        Ok(())
    }

    fn skip_spaces(&mut self) -> Result<(), CsarError> {
        while let Some(next) = self.chars.peek() {
            match next {
                Ok(next) => {
                    if *next == ' ' {
                        self.skip()?;
                    } else {
                        break;
                    }
                }

                Err(error) => return Err(ToscaMetaError::Malformed(error.to_string()).into()),
            }
        }

        Ok(())
    }

    fn continues(&mut self) -> Result<bool, CsarError> {
        match self.chars.peek() {
            Some(next) => match next {
                Ok(next) => {
                    // Line starts with a space?
                    if *next == ' ' {
                        self.skip_spaces()?;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }

                Err(error) => Err(ToscaMetaError::Malformed(error.to_string()).into()),
            },

            None => Ok(false),
        }
    }
}

enum KeyValueMode {
    Key,
    Value,
}
