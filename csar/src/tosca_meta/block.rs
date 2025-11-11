use super::{super::errors::*, syntax::*, version::*};

use {compris::normal::*, depiction::*, indexmap::map::*, kutil::std::error::*, std::fmt};

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
    pub fn must_get<ErrorReceiverT>(
        &self,
        keyname: &str,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<&String>, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
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
    pub fn must_get_version<ErrorReceiverT>(
        &self,
        keyname: &str,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<Version>, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        Ok(match self.must_get(keyname, errors)? {
            Some(value) => ok_give!(Version::parse(keyname, value), errors),
            None => None,
        })
    }

    /// Get list of strings or error if it's malformed.
    pub fn get_list<ErrorReceiverT>(
        &self,
        keyname: &str,
        errors: &mut ErrorReceiverT,
    ) -> Result<Option<Vec<String>>, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        Ok(match self.values.get(keyname) {
            Some(value) => ok_give!(string_list_from_tosca_meta(keyname, value), errors),
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
