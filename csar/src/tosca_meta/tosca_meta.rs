use super::{super::errors::*, block::*, syntax::*, version::*};

use {
    compris::normal::*,
    depiction::*,
    kutil::std::error::*,
    std::{fs::*, io, path::*},
};

//
// ToscaMeta
//

/// TOSCA meta in CSAR.
#[derive(Clone, Debug, Depict)]
pub struct ToscaMeta {
    /// This is the version number of the CSAR specification. It defines the structure of the CSAR
    /// and the format of the TOSCA.meta file. The value MUST be "2.0" for this version of the CSAR
    /// specification.
    #[depict(as(depict))]
    pub csar_version: Version,

    /// The person or organization that created the CSAR.
    #[depict(option, as(debug), style(string))]
    pub created_by: Option<String>,

    /// This references the TOSCA definitions file that SHOULD be used as entry point for
    /// processing the contents of the CSAR (e.g. the main TOSCA service template).
    #[depict(option, as(debug), style(string))]
    pub entry_definitions: Option<String>,

    /// This references an unambiguous set of files containing substitution templates that can be
    /// used to implement nodes defined in the main template (i.e. the file declared in
    /// Entry-Definitions). Thus, all the service templates defined in files listed under the
    /// Other-Definitions key are to be used only as substitution templates, and not as standalone
    /// services. If such a service template cannot act as a substitution template, it will be
    /// ignored by the orchestrator. The value of the Other-Definitions key is a string containing
    /// a list of filenames (relative to the root of the CSAR archive) delimited by a blank space.
    /// If the filenames contain blank spaces, the filename should be enclosed by double quotation
    /// marks (").
    #[depict(iter(item), as(debug), style(string))]
    pub other_definitions: Vec<String>,

    /// Blocks after `block_0`.
    #[depict(iter(item), as(depict))]
    pub extra_blocks: Vec<ToscaMetaBlock>,
}

impl ToscaMeta {
    /// Read.
    pub fn read<ReadT, ErrorReceiverT>(read: &mut ReadT, errors: &mut ErrorReceiverT) -> Result<Self, CsarError>
    where
        ReadT: io::BufRead,
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        let mut tosca_meta = Self::default();

        let mut reader = ToscaMetaBlockReader::new_from(read);
        let mut index = 0;
        while let Some(block) = must_unwrap_give!(reader.read_block(), errors, tosca_meta) {
            if index == 0 {
                tosca_meta.parse_block_0(block, true, errors)?;
            } else {
                tosca_meta.extra_blocks.push(block);
            }

            index += 1;
        }

        if !reader.is_empty() {
            errors.give(ToscaMetaError::Malformed("has text after last block".into()))?;
        }

        Ok(tosca_meta)
    }

    /// Read from path.
    pub fn read_path<PathT, ErrorReceiverT>(path: PathT, errors: &mut ErrorReceiverT) -> Result<Self, CsarError>
    where
        PathT: AsRef<Path>,
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        Self::read(&mut io::BufReader::new(File::open(path)?), errors)
    }

    /// From bytes.
    pub fn from_bytes<ErrorReceiverT>(bytes: &[u8], errors: &mut ErrorReceiverT) -> Result<Self, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        Self::read(&mut io::Cursor::new(bytes), errors)
    }

    /// From string.
    pub fn from_string<ErrorReceiverT>(string: &str, errors: &mut ErrorReceiverT) -> Result<Self, CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        Self::from_bytes(string.as_bytes(), errors)
    }

    /// Stringify.
    pub fn stringify(&self, max_columns: Option<usize>) -> Result<String, CsarError> {
        let mut string = String::default();

        let block_0: ToscaMetaBlock = self.try_into()?;
        string += &block_0.stringify(max_columns);

        for block in &self.extra_blocks {
            string += "\n";
            string += &block.stringify(max_columns);
        }

        Ok(string)
    }

    fn parse_block_0<ErrorReceiverT>(
        &mut self,
        block_0: ToscaMetaBlock,
        validate_version: bool,
        errors: &mut ErrorReceiverT,
    ) -> Result<(), CsarError>
    where
        ErrorReceiverT: ErrorReceiver<CsarError>,
    {
        if let Some(version) = block_0.must_get_version("CSAR-Version", errors)? {
            self.csar_version = version;
        }

        if validate_version && (self.csar_version != Version::new(2, 0)) {
            return Err(ToscaMetaError::UnsupportedVersion(self.csar_version.clone()).into());
        }

        for keyname in block_0.keynames() {
            match keyname.as_str() {
                "CSAR-Version" | "Created-By" | "Entry-Definitions" | "Other-Definitions" => {}
                _ => errors.give(ToscaMetaError::UnsupportedKeyname(keyname.clone()))?,
            }
        }

        self.created_by = block_0.get("Created-By").cloned();
        self.entry_definitions = block_0.get("Entry-Definitions").cloned();

        if let Some(other_definitions) = block_0.get_list("Other-Definitions", errors)? {
            self.other_definitions = other_definitions;
        }

        Ok(())
    }

    /// Complete missing fields.
    pub fn complete(mut self) -> Self {
        if self.created_by.is_none() {
            self.created_by = Some("Puccini".into());
        }
        self
    }
}

impl Default for ToscaMeta {
    fn default() -> Self {
        Self {
            csar_version: Version::new(2, 0),
            created_by: Some("Puccini".into()),
            entry_definitions: None,
            other_definitions: Default::default(),
            extra_blocks: Default::default(),
        }
    }
}

impl TryFrom<&ToscaMeta> for ToscaMetaBlock {
    type Error = CsarError;

    fn try_from(tosca_meta: &ToscaMeta) -> Result<Self, Self::Error> {
        let mut block_0 = ToscaMetaBlock::default();

        block_0.insert("CSAR-Version".into(), tosca_meta.csar_version.to_string())?;

        if let Some(created_by) = &tosca_meta.created_by {
            block_0.insert("Created-By".into(), created_by.clone())?;
        }

        if let Some(entry_definitions) = &tosca_meta.entry_definitions {
            block_0.insert("Entry-Definitions".into(), entry_definitions.clone())?;
        }

        if !tosca_meta.other_definitions.is_empty() {
            block_0.insert_list("Other-Definitions".into(), &tosca_meta.other_definitions)?;
        }

        Ok(block_0)
    }
}

impl<AnnotatedT> Into<Variant<AnnotatedT>> for ToscaMeta
where
    AnnotatedT: Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        let mut map = Map::default();

        map.into_insert("csar-version", self.csar_version.to_string());

        if let Some(created_by) = self.created_by {
            map.into_insert("created-by", created_by);
        }

        if let Some(entry_definitions) = self.entry_definitions {
            map.into_insert("entry-definitions", entry_definitions);
        }

        if !self.other_definitions.is_empty() {
            let other_definitions: List<_> = self.other_definitions.into_iter().map(|value| value.into()).collect();
            map.into_insert("other-definitions", other_definitions);
        }

        if !self.extra_blocks.is_empty() {
            let extra_blocks: List<_> = self.extra_blocks.into_iter().map(|value| value.into()).collect();
            map.into_insert("extra-blocks", extra_blocks);
        }

        map.into()
    }
}
