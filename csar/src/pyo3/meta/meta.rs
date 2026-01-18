use super::{super::super::tosca_meta::*, block::*};

use {compris::pyo3::problemo::*, depiction::*, kutil::pyo3::*, problemo::*, pyo3::prelude::*, std::io};

//
// PyToscaMeta
//

/// TOSCA meta.
#[pyclass(name = "Meta")]
pub struct PyToscaMeta {
    /// Inner TOSCA meta.
    pub inner: ToscaMeta,
}

#[pymethods]
impl PyToscaMeta {
    /// Constructor.
    #[new]
    #[pyo3(signature = (content=None))]
    pub fn new<'py>(content: Option<&Bound<'py, PyAny>>) -> PyResult<Self> {
        Ok(match content {
            Some(content) => {
                let mut problems = Problems::default();

                let meta = match ReadableAny::from(content) {
                    ReadableAny::Bytes(bytes) => ToscaMeta::from_bytes(bytes, &mut problems),
                    ReadableAny::String(string) => ToscaMeta::from_string(string, &mut problems),
                    ReadableAny::FileLike(file_like) => {
                        ToscaMeta::read(&mut io::BufReader::new(file_like), &mut problems)
                    }
                }
                .into_py()?;

                problems.check().into_py()?;
                meta.into()
            }

            None => ToscaMeta::default().into(),
        })
    }

    /// This is the version number of the CSAR specification. It defines the structure of the CSAR
    /// and the format of the TOSCA.meta file. The value MUST be "2.0" for this version of the CSAR
    /// specification.
    #[getter]
    pub fn csar_version(&self) -> String {
        self.inner.csar_version.to_string()
    }

    /// See [PyToscaMeta::csar_version].
    #[setter]
    pub fn set_csar_version(&mut self, csar_version: &str) -> PyResult<()> {
        self.inner.csar_version = Version::parse("CSAR-Version", csar_version).into_py()?;
        Ok(())
    }

    /// The person or organization that created the CSAR.
    #[getter]
    pub fn created_by(&self) -> Option<String> {
        self.inner.created_by.clone()
    }

    /// See [PyToscaMeta::created_by].
    #[setter]
    pub fn set_created_by(&mut self, created_by: String) {
        self.inner.created_by = Some(created_by);
    }

    /// This references the TOSCA definitions file that SHOULD be used as entry point for
    /// processing the contents of the CSAR (e.g. the main TOSCA service template).
    #[getter]
    pub fn entry_definitions(&self) -> Option<String> {
        self.inner.entry_definitions.clone()
    }

    /// See [PyToscaMeta::entry_definitions].
    #[setter]
    pub fn set_entry_definitions(&mut self, entry_definitions: String) {
        self.inner.entry_definitions = Some(entry_definitions);
    }

    /// This references an unambiguous set of files containing substitution templates that can be
    /// used to implement nodes defined in the main template (i.e. the file declared in
    /// Entry-Definitions). Thus, all the service templates defined in files listed under the
    /// Other-Definitions key are to be used only as substitution templates, and not as standalone
    /// services. If such a service template cannot act as a substitution template, it will be
    /// ignored by the orchestrator. The value of the Other-Definitions key is a string containing
    /// a list of filenames (relative to the root of the CSAR archive) delimited by a blank space.
    /// If the filenames contain blank spaces, the filename should be enclosed by double quotation
    /// marks (").
    #[getter]
    pub fn other_definitions(&self) -> Vec<String> {
        self.inner.other_definitions.clone()
    }

    /// See [PyToscaMeta::other_definitions].
    #[setter]
    pub fn set_other_definitions(&mut self, other_definitions: Vec<String>) {
        self.inner.other_definitions = other_definitions;
    }

    /// Blocks after `block_0`.
    pub fn get_extra_blocks(&self) -> Vec<PyToscaMetaBlock> {
        self.inner.extra_blocks.iter().map(|block| block.clone().into()).collect()
    }

    /// See [PyToscaMeta::get_extra_blocks].
    pub fn set_extra_blocks<'py>(&mut self, extra_blocks: Vec<Bound<'py, PyToscaMetaBlock>>) {
        self.inner.extra_blocks = extra_blocks.into_iter().map(|block| block.borrow().inner.clone()).collect();
    }

    /// As raw string.
    #[pyo3(signature = (max_columns=None))]
    pub fn raw(&self, max_columns: Option<usize>) -> PyResult<String> {
        self.inner.stringify(max_columns).into_py()
    }

    /// Format.
    pub fn __format__(&self, specification: Option<&str>) -> PyResult<String> {
        if let Some(specification) = specification {
            let mut tags = FormatSpecificationTags::from(specification);
            if tags.remove("raw") {
                return self.inner.stringify(None).into_py();
            } else if let Some(max_columns) = tags.remove_prefix("raw.") {
                let max_columns = max_columns.parse()?;
                return self.inner.stringify(Some(max_columns)).into_py();
            }
        }

        self.py_format(specification)
    }

    /// As string.
    pub fn __str__(&self) -> PyResult<String> {
        self.py_str()
    }
}

impl ToDepiction for PyToscaMeta {
    fn to_depiction(&self, context: &DepictionContext) -> io::Result<String> {
        self.inner.to_depiction(context)
    }
}

impl From<ToscaMeta> for PyToscaMeta {
    fn from(inner: ToscaMeta) -> Self {
        Self { inner }
    }
}
