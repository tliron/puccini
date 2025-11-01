use super::{super::errors::*, syntax::*, version::*};

use {
    compris::normal::*,
    indexmap::map::*,
    kutil::{cli::depict::*, std::error::*},
    std::fmt,
};

//
// ToscaMetaBlock
//

/// Block in a [ToscaMeta](super::ToscaMeta).
///
/// The order of keys is preserved for roundtrips.
#[derive(Clone, Debug, Default, Depict)]
pub struct ToscaMetaBlock {
    #[depict(iter(kv), key_as(debug), as(debug), key_style(name), style(string))]
    values: IndexMap<String, String>,
}

impl ToscaMetaBlock {
    /// Keynames.
    pub fn keynames(&self) -> Keys<'_, String, String> {
        self.values.keys()
    }

    /// Get value.
    pub fn get(&self, keyname: &str) -> Option<&String> {
        self.values.get(keyname)
    }

    /// Get value or error if it doesn't exist.
    pub fn must_get<ErrorRecipientT>(
        &self,
        keyname: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<&String>, CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        Ok(match self.values.get(keyname) {
            Some(value) => Some(value),
            None => {
                errors.give(ToscaMetaError::RequiredKeyname(keyname.into()))?;
                None
            }
        })
    }

    /// Get version value or error if it doesn't exist.
    pub fn must_get_version<ErrorRecipientT>(
        &self,
        keyname: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<Version>, CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        Ok(match self.must_get(keyname, errors)? {
            Some(value) => match Version::parse(keyname, value) {
                Ok(version) => Some(version),
                Err(error) => {
                    errors.give(error)?;
                    None
                }
            },

            None => None,
        })
    }

    /// Get list of strings or error if it's malformed.
    pub fn get_list<ErrorRecipientT>(
        &self,
        keyname: &str,
        errors: &mut ErrorRecipientT,
    ) -> Result<Option<Vec<String>>, CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        Ok(match self.values.get(keyname) {
            Some(value) => match string_list_from_tosca_meta(keyname, value) {
                Ok(strings) => Some(strings),
                Err(error) => {
                    errors.give(error)?;
                    None
                }
            },

            None => None,
        })
    }

    /// Insert.
    pub fn insert(&mut self, keyname: String, value: String) -> Result<(), CsarError> {
        if keyname.contains(':') {
            return Err(InvalidKeyError::new(keyname, "keyname contains `:`".into()).into());
        }

        self.values.insert(keyname, value);
        Ok(())
    }

    /// Insert list of strings.
    pub fn insert_list(&mut self, keyname: String, values: &Vec<String>) -> Result<(), CsarError> {
        let list = string_list_to_tosca_meta(&keyname, values)?;
        self.insert(keyname, list.join(" "))
    }

    /// Stringify.
    pub fn stringify(&self, max_columns: Option<usize>) -> String {
        // No need to test for validity because we don't allow invalid keys into the block.
        keys_to_tosca_meta(self.values.iter(), max_columns)
    }
}

impl fmt::Display for ToscaMetaBlock {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.stringify(None), formatter)
    }
}

impl<AnnotatedT> Into<Variant<AnnotatedT>> for ToscaMetaBlock
where
    AnnotatedT: Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        self.values.into_iter().map(|(key, value)| (key.into(), value.into())).collect()
    }
}
