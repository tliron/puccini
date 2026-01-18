use super::{super::super::creator::*, created::*, writer::*};

use {
    compris::pyo3::problemo::*,
    kutil::pyo3::*,
    problemo::*,
    pyo3::prelude::*,
    std::{io, path::*},
};

//
// PyCsarCreator
//

/// CSAR creator.
#[pyclass(name = "Creator")]
pub struct PyCsarCreator {
    /// Inner archive writer.
    pub inner: CsarCreator,
}

#[pymethods]
impl PyCsarCreator {
    /// Constructor.
    #[new]
    pub fn new() -> Self {
        let mut creator = CsarCreator::default();
        creator.read_tracker = Some(Box::new(DummyReadTracker));
        creator.into()
    }

    /// Format.
    #[getter]
    pub fn format(&self) -> Option<String> {
        self.inner.format.map(|format| format.to_string())
    }

    /// See [PyCsarCreator::format].
    #[setter]
    pub fn set_format(&mut self, format: &str) -> PyResult<()> {
        self.inner.format = Some(format.parse::<Format>().into_py()?);
        Ok(())
    }

    /// Compression level.
    #[getter]
    pub fn compression_level(&self) -> Option<usize> {
        self.inner.compression_level.map(|compression_level| compression_level.into())
    }

    /// See [PyCsarCreator::compression_level].
    #[setter]
    pub fn set_compression_level(&mut self, compression_level: usize) -> PyResult<()> {
        self.inner.compression_level = Some(CompressionLevel::try_from(compression_level).into_py()?);
        Ok(())
    }

    /// Created by.
    #[getter]
    pub fn created_by(&self) -> Option<String> {
        self.inner.created_by.clone()
    }

    /// See [PyCsarCreator::created_by].
    #[setter]
    pub fn set_created_by(&mut self, created_by: String) {
        self.inner.created_by = Some(created_by);
    }

    /// Entry definitions.
    #[getter]
    pub fn entry_definitions(&self) -> Option<String> {
        self.inner.entry_definitions.clone()
    }

    /// See [PyCsarCreator::entry_definitions].
    #[setter]
    pub fn set_entry_definitions(&mut self, entry_definitions: String) {
        self.inner.entry_definitions = Some(entry_definitions);
    }

    /// Additional other definitions.
    #[getter]
    pub fn additional_other_definitions(&self) -> Vec<String> {
        self.inner.additional_other_definitions.clone()
    }

    /// See [PyCsarCreator::additional_other_definitions].
    #[setter]
    pub fn set_additional_other_definitions(&mut self, additional_other_definitions: Vec<String>) {
        self.inner.additional_other_definitions = additional_other_definitions;
    }

    /// Maximum number of columns for formatting `TOSCA.meta` file.
    #[getter]
    pub fn max_columns(&self) -> Option<usize> {
        self.inner.max_columns
    }

    /// See [PyCsarCreator::max_columns].
    #[setter]
    pub fn set_max_columns(&mut self, max_columns: usize) {
        self.inner.max_columns = Some(max_columns);
    }

    /// Create CSAR file from directory.
    ///
    /// If the format is not provided it will be selected according to the archive extension or default
    /// to [Format::Tarball].
    pub fn create_from_directory(&self, file: PathBuf, directory: PathBuf) -> PyResult<Option<PyCreatedCsar>> {
        let mut problems = Problems::default();
        let created_csar =
            self.inner.create_from_directory(Some(&file), &directory, false, false, false, &mut problems).into_py()?;
        problems.check().into_py()?;
        Ok(created_csar.map(|created| created.into()))
    }

    /// Writer.
    pub fn writer(&self, writer: Bound<'_, PyAny>) -> PyResult<PyCsarArchiveWriter> {
        let seekable = is_file_like_seekable(&writer)?;

        let writer = PyBinaryFile::from(writer);
        let mut writer: PyCsarArchiveWriter = if seekable {
            self.inner.seek_writer(writer).into_py()?.into()
        } else {
            self.inner.stream_writer(writer).into_py()?.into()
        };
        writer.compression_level = self.inner.compression_level;

        Ok(writer)
    }

    /// Buffer writer.
    #[pyo3(signature = (initial_capacity=None))]
    pub fn buffer_writer(&self, initial_capacity: Option<usize>) -> PyResult<PyCsarArchiveWriter> {
        let buffer = match initial_capacity {
            Some(initial_capacity) => Vec::with_capacity(initial_capacity),
            None => Vec::default(),
        };

        let mut writer: PyCsarArchiveWriter = self.inner.seek_writer(io::Cursor::new(buffer)).into_py()?.into();
        writer.compression_level = self.inner.compression_level;

        Ok(writer)
    }
}

impl From<CsarCreator> for PyCsarCreator {
    fn from(inner: CsarCreator) -> Self {
        Self { inner }
    }
}
