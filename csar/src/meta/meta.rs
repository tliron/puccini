use super::{super::errors::*, block::*, syntax::*, version::*};

use {
    compris::normal::*,
    kutil::{cli::depict::*, std::error::*},
    std::{fs::*, io, path::*},
};

/// `TOSCA.meta` artifact path.
pub const TOSCA_META_PATH_1: &str = "TOSCA.meta";

/// `TOSCA.meta` artifact path.
pub const TOSCA_META_PATH_2: &str = "TOSCA-Metadata/TOSCA.meta";

//
// Meta
//

/// CSAR meta.
#[derive(Clone, Debug, Depict)]
pub struct Meta {
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
    pub extra_blocks: Vec<MetaBlock>,
}

impl Meta {
    /// Read.
    pub fn read<ReadT, ErrorRecipientT>(read: &mut ReadT, errors: &mut ErrorRecipientT) -> Result<Self, CsarError>
    where
        ReadT: io::BufRead,
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        let mut meta = Self::default();

        let mut reader = MetaBlockReader::new_from(read);
        let mut index = 0;
        while let Some(block) = unwrap_or_give_and_return!(reader.read_block(), errors, Ok(meta)) {
            if index == 0 {
                meta.parse_block_0(block, errors)?;
            } else {
                meta.extra_blocks.push(block);
            }

            index += 1;
        }

        if !reader.is_empty() {
            errors.give(MetaError::Malformed("has text after last block".into()))?;
        }

        Ok(meta)
    }

    /// Read from path.
    pub fn read_path<PathT, ErrorRecipientT>(path: PathT, errors: &mut ErrorRecipientT) -> Result<Self, CsarError>
    where
        PathT: AsRef<Path>,
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        Self::read(&mut io::BufReader::new(File::open(path)?), errors)
    }

    /// From bytes.
    pub fn from_bytes<ErrorRecipientT>(bytes: &[u8], errors: &mut ErrorRecipientT) -> Result<Self, CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        Self::read(&mut io::Cursor::new(bytes), errors)
    }

    /// From string.
    pub fn from_string<ErrorRecipientT>(string: &str, errors: &mut ErrorRecipientT) -> Result<Self, CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        Self::from_bytes(string.as_bytes(), errors)
    }

    /// Stringify.
    pub fn stringify(&self, max_columns: Option<usize>) -> Result<String, CsarError> {
        let mut string = String::default();

        let block_0: MetaBlock = self.try_into()?;
        string += &block_0.stringify(max_columns);

        for block in &self.extra_blocks {
            string += "\n";
            string += &block.stringify(max_columns);
        }

        Ok(string)
    }

    /// Validate paths.
    pub fn validate_paths<ErrorRecipientT>(
        &self,
        directory: &Path,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        if let Some(entry_definitions) = &self.entry_definitions {
            let entry_definitions = directory.join(entry_definitions);
            if !entry_definitions.exists() {
                errors.give(MetaError::FileNotFound(entry_definitions.display().to_string()))?;
            }
        }

        for other_definition in &self.other_definitions {
            let other_definition = directory.join(other_definition);
            if !other_definition.exists() {
                errors.give(MetaError::FileNotFound(other_definition.display().to_string()))?;
            }
        }

        Ok(())
    }

    fn parse_block_0<ErrorRecipientT>(
        &mut self,
        block: MetaBlock,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), CsarError>
    where
        ErrorRecipientT: ErrorRecipient<CsarError>,
    {
        if let Some(version) = block.must_get_version("CSAR-Version", errors)? {
            self.csar_version = version;
        }

        if self.csar_version != Version::new(2, 0) {
            return Err(MetaError::UnsupportedVersion(self.csar_version.clone()).into());
        }
        for keyname in block.keynames() {
            match keyname.as_str() {
                "CSAR-Version" | "Created-By" | "Entry-Definitions" | "Other-Definitions" => {}
                _ => errors.give(MetaError::UnsupportedKeyname(keyname.clone()))?,
            }
        }

        self.created_by = block.get("Created-By").cloned();
        self.entry_definitions = block.get("Entry-Definitions").cloned();

        if let Some(other_definitions) = block.get_list("Other-Definitions", errors)? {
            self.other_definitions = other_definitions;
        }

        Ok(())
    }
}

impl Default for Meta {
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

impl TryFrom<&Meta> for MetaBlock {
    type Error = CsarError;

    fn try_from(meta: &Meta) -> Result<Self, Self::Error> {
        let mut block = MetaBlock::default();

        block.insert("CSAR-Version".into(), meta.csar_version.to_string())?;

        if let Some(created_by) = &meta.created_by {
            block.insert("Created-By".into(), created_by.clone())?;
        }

        if let Some(entry_definitions) = &meta.entry_definitions {
            block.insert("Entry-Definitions".into(), entry_definitions.clone())?;
        }

        if !meta.other_definitions.is_empty() {
            block.insert_list("Other-Definitions".into(), &meta.other_definitions)?;
        }

        Ok(block)
    }
}

impl<AnnotatedT> Into<Variant<AnnotatedT>> for Meta
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
