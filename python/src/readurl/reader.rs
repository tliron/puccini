use {
    pyo3::{buffer::*, exceptions::*, prelude::*, types::*},
    read_url::*,
    std::io::{self, BufRead, Read},
};

// https://docs.python.org/3/library/io.html#io.IOBase

//
// PyUrlReader
//

/// URL.
#[pyclass(name = "Reader")]
pub struct PyUrlReader {
    /// Inner reader reference.
    pub inner: Option<io::BufReader<ReadRef>>,
}

#[pymethods]
impl PyUrlReader {
    /// Close.
    pub fn close(&mut self) {
        self.inner.take();
    }

    /// True if closed.
    #[getter]
    pub fn closed(&self) -> bool {
        self.inner.is_none()
    }

    /// Read bytes.
    #[pyo3(signature = (size=-1))]
    pub fn read(&mut self, size: i64) -> PyResult<Option<Vec<u8>>> {
        match size {
            0 => {
                self.inner()?;
                Ok(None)
            }

            -1 => self.readall(),

            _ => {
                let mut bytes = Vec::with_capacity(size as usize);
                Ok(match self.inner()?.take(size as u64).read(&mut bytes)? {
                    0 => None,
                    _ => Some(bytes),
                })
            }
        }
    }

    /// Read all bytes.
    pub fn readall(&mut self) -> PyResult<Option<Vec<u8>>> {
        let mut bytes = Vec::default();
        Ok(match self.inner()?.read_to_end(&mut bytes)? {
            0 => None,
            _ => Some(bytes),
        })
    }

    /// Read a line.
    #[pyo3(signature = (size=-1))]
    pub fn readline(&mut self, size: isize) -> PyResult<Option<String>> {
        match size {
            0 => {
                self.inner()?;
                Ok(None)
            }

            -1 => {
                let mut line = String::default();
                Ok(match self.inner()?.read_line(&mut line)? {
                    0 => None,
                    _ => Some(line),
                })
            }

            _ => {
                let mut line = String::with_capacity(size as usize);
                Ok(match self.inner()?.take(size as u64).read_line(&mut line)? {
                    0 => None,
                    _ => Some(line),
                })
            }
        }
    }

    /// Read into bytes.
    pub fn readinto(&mut self, bytes: &Bound<'_, PyAny>) -> PyResult<usize> {
        let py_buffer = PyBuffer::<u8>::get(bytes)?;
        let size = py_buffer.item_count();

        // Quite inefficient :(
        // Access to the buffer is &mut [Cell], which is awkward...
        // The fastest way to write is apparently via copy_from_slice,
        // which requires the source to be the same size

        let mut buffer = Vec::with_capacity(size);
        let count = self.inner()?.take(size as u64).read_to_end(&mut buffer)?;
        buffer.resize(size, 0);
        py_buffer.copy_from_slice(bytes.py(), &buffer)?;

        Ok(count)
    }

    /// True if readable.
    pub fn readable(&self) -> bool {
        self.inner.is_some()
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
        Err(PyIOError::new_err("not a file"))
    }

    /// True if TTY.
    pub fn isatty(&self) -> bool {
        false
    }

    /// Flush.
    pub fn flush(&self) {}

    /// Iterator.
    pub fn __iter__(self_: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if self_.inner.is_some() { Ok(self_) } else { Err(PyIOError::new_err("closed")) }
    }

    /// Next line.
    pub fn __next__(&mut self) -> PyResult<Option<String>> {
        self.readline(-1)
    }

    /// Context manager enter.
    pub fn __enter__(self_: PyRef<'_, Self>) -> PyResult<PyRef<'_, Self>> {
        if self_.inner.is_some() { Ok(self_) } else { Err(PyIOError::new_err("closed")) }
    }

    /// Context manager exit.
    pub fn __exit__<'py>(
        &mut self,
        _type: Option<Bound<'py, PyType>>,
        _value: Option<Bound<'py, PyAny>>,
        _traceback: Option<Bound<'py, PyAny>>,
    ) -> PyResult<bool> {
        self.close();
        Ok(false)
    }
}

impl PyUrlReader {
    fn inner(&mut self) -> PyResult<&mut io::BufReader<ReadRef>> {
        self.inner.as_mut().ok_or_else(|| PyIOError::new_err("closed"))
    }
}

impl From<ReadRef> for PyUrlReader {
    fn from(inner: ReadRef) -> Self {
        Self { inner: Some(io::BufReader::new(inner)) }
    }
}
