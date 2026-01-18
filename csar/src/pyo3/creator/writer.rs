use super::super::super::creator::*;

use {
    compris::pyo3::problemo::*,
    kutil::pyo3::*,
    problemo::common::*,
    pyo3::{exceptions::*, prelude::*, types::*},
    std::{io, path::*, sync::*},
};

// https://docs.python.org/3/library/io.html#io.IOBase

//
// PyCsarArchiveWriter
//

/// CSAR archive writer.
#[pyclass(name = "ArchiveWriter")]
pub struct PyCsarArchiveWriter {
    /// Inner archive writer.
    pub inner: Mutex<Option<ArchiveWriterWrapper>>,

    /// Compression level.
    pub compression_level: Option<CompressionLevel>,
}

#[pymethods]
impl PyCsarArchiveWriter {
    /// Create a new archive entry.
    #[pyo3(signature = (name, source, size=None))]
    pub fn add<'py>(&mut self, name: PathBuf, source: &Bound<'py, PyAny>, size: Option<usize>) -> PyResult<()> {
        let mut writer = self.inner()?;
        let Some(writer) = writer.as_mut() else {
            return Err(closed());
        };

        match ReadableAny::from(source) {
            ReadableAny::Bytes(bytes) => writer.add_bytes(name, bytes, self.compression_level, None),
            ReadableAny::String(string) => writer.add_string(name, string, self.compression_level, None),

            ReadableAny::FileLike(mut file_like) => {
                let Some(size) = size else {
                    return Err(PyException::new_err("missing size"));
                };

                writer.add_from_reader(&name, Box::new(&mut file_like), size, self.compression_level, None)
            }
        }
        .into_py()
    }

    /// Create a new archive entry from a file.
    pub fn add_file(&mut self, name: PathBuf, source: PathBuf) -> PyResult<()> {
        let mut writer = self.inner()?;
        let Some(writer) = writer.as_mut() else {
            return Err(closed());
        };

        writer.add_file(name, source, self.compression_level, None).into_py()
    }

    /// Buffer.
    ///
    /// Calling this will close the writer.
    pub fn buffer(&mut self) -> PyResult<Vec<u8>> {
        match self.take_inner()?.into_writer::<io::Cursor<_>>() {
            Some(buffer) => Ok(buffer.into_inner()),
            None => Err(PyTypeError::new_err("not a buffer")),
        }
    }

    /// Close.
    pub fn close(&mut self) -> PyResult<()> {
        self.take_inner()?;
        Ok(())
    }

    /// True if closed.
    #[getter]
    pub fn closed(&self) -> PyResult<bool> {
        Ok(self.inner()?.is_none())
    }

    /// True if readable.
    pub fn readable(&self) -> bool {
        false
    }

    /// True if seekable.
    pub fn seekable(&self) -> bool {
        false
    }

    /// True if writable.
    pub fn writable(&self) -> bool {
        false
    }

    /// Fileno.
    pub fn fileno(&self) -> PyResult<()> {
        Err(PyTypeError::new_err("not a file"))
    }

    /// True if TTY.
    pub fn isatty(&self) -> bool {
        false
    }

    /// Flush.
    pub fn flush(&self) -> PyResult<()> {
        let mut writer = self.inner()?;
        let Some(writer) = writer.as_mut() else {
            return Err(closed());
        };

        writer.flush().into_py()
    }

    /// Context manager enter.
    pub fn __enter__(self_: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if self_.closed()? { Err(closed()) } else { Ok(self_) }
    }

    /// Context manager exit.
    pub fn __exit__<'py>(
        &mut self,
        _type: Option<Bound<'py, PyType>>,
        _value: Option<Bound<'py, PyAny>>,
        _traceback: Option<Bound<'py, PyAny>>,
    ) -> PyResult<bool> {
        self.close()?;
        Ok(false)
    }
}

impl PyCsarArchiveWriter {
    fn inner(&self) -> PyResult<MutexGuard<'_, Option<ArchiveWriterWrapper>>> {
        self.inner.lock().into_thread_problem().into_py()
    }

    fn take_inner(&self) -> PyResult<ArchiveWriterWrapper> {
        match self.inner.lock().into_thread_problem().into_py()?.take() {
            Some(writer) => Ok(writer),
            None => Err(closed()),
        }
    }
}

impl<ArchiveWriterT> From<ArchiveWriterT> for PyCsarArchiveWriter
where
    ArchiveWriterT: Into<ArchiveWriterWrapper>,
{
    fn from(inner: ArchiveWriterT) -> Self {
        Self { inner: Mutex::new(Some(inner.into())), compression_level: None }
    }
}

// Utils

fn closed() -> PyErr {
    PyIOError::new_err("closed")
}
