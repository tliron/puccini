use problemo::common::NotFoundError;

use super::{super::errors::*, syntax::*, version::*};

use {compris::normal::*, depiction::*, indexmap::map::*, problemo::*, std::fmt};

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
    /// Length.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Keynames.
    pub fn keynames(&self) -> Keys<'_, String, String> {
        self.values.keys()
    }

    /// Get value.
    pub fn get(&self, keyname: &str) -> Option<&String> {
        self.values.get(keyname)
    }

    /// Get value or error if it doesn't exist.
    pub fn must_get<ProblemReceiverT>(
        &self,
        keyname: &str,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<&String>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(match self.values.get(keyname) {
            Some(value) => Some(value),
            None => {
                problems.give(RequiredKeynameError::as_problem(keyname).via(CsarError))?;
                None
            }
        })
    }

    /// Get version value or error if it doesn't exist.
    pub fn must_get_version<ProblemReceiverT>(
        &self,
        keyname: &str,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<Version>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(match self.must_get(keyname, problems)? {
            Some(value) => Version::parse(keyname, value).give_ok(problems)?,
            None => None,
        })
    }

    /// Get list of strings or error if it's malformed.
    pub fn get_list<ProblemReceiverT>(
        &self,
        keyname: &str,
        problems: &mut ProblemReceiverT,
    ) -> Result<Option<Vec<String>>, Problem>
    where
        ProblemReceiverT: ProblemReceiver,
    {
        Ok(match self.values.get(keyname) {
            Some(value) => string_list_from_tosca_meta(keyname, value).give_ok(problems)?,
            None => None,
        })
    }

    /// Insert.
    pub fn insert(&mut self, keyname: String, value: String) -> Result<(), Problem> {
        if keyname.contains(':') {
            return Err(InvalidKeyError::as_problem(keyname, "keyname contains `:`".into()).via(CsarError));
        }

        self.values.insert(keyname, value);
        Ok(())
    }

    /// Insert list of strings.
    pub fn insert_list(&mut self, keyname: String, values: &Vec<String>) -> Result<(), Problem> {
        let list = string_list_to_tosca_meta(&keyname, values)?;
        self.insert(keyname, list.join(" "))
    }

    /// Remove.
    pub fn remove(&mut self, keyname: &str) -> Result<(), Problem> {
        match self.values.swap_remove(keyname) {
            Some(_) => Err(NotFoundError::as_problem(keyname).via(CsarError)),
            None => Ok(()),
        }
    }

    /// Stringify.
    pub fn stringify(&self, max_columns: Option<usize>) -> String {
        // No need to test for validity because we don't allow invalid keys into the block.
        keys_to_tosca_meta(self.values.iter(), max_columns)
    }
}

impl fmt::Display for ToscaMetaBlock {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
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
