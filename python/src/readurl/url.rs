use super::{super::problemo::*, reader::*};

use {
    problemo::common::*,
    pyo3::{class::basic::*, prelude::*},
    read_url::*,
    std::{hash::*, path::*, sync::*},
};

/// Format an archive URL.
#[pyfunction(name = "format_archive_url")]
pub fn py_format_archive_url(scheme: &str, archive: &str, path: &str) -> String {
    format_archive_url(scheme, archive, path)
}

//
// PyUrl
//

/// URL.
#[pyclass(name = "URL")]
pub struct PyUrl {
    /// Inner URL reference.
    pub inner: Mutex<UrlRef>,
}

#[pymethods]
impl PyUrl {
    /// Returns a string that uniquely identifies the URL.
    ///
    /// Useful as a map or cache key.
    #[getter]
    pub fn key(&self) -> PyResult<String> {
        Ok(self.inner()?.key())
    }

    /// The optional query.
    #[getter]
    pub fn query(&self) -> PyResult<Option<UrlQuery>> {
        Ok(self.inner()?.query())
    }

    /// The optional fragment.
    #[getter]
    pub fn fragment(&self) -> PyResult<Option<String>> {
        Ok(self.inner()?.fragment())
    }

    /// Format of the URL content's canonical representation.
    ///
    /// Can return "text", "yaml", "json", "tar", "tar.gz", etc.
    ///
    /// The format is often derived from a file extension if available, otherwise
    /// it might be retrieved from metadata.
    ///
    /// An attempt is made to standardize the return values, e.g. a "yml" file
    /// extension is always returned as "yaml", and a "tgz" file extension is
    /// always returned as "tar.gz".
    #[getter]
    pub fn format(&self) -> PyResult<Option<String>> {
        Ok(self.inner()?.format())
    }

    /// If this URL points to a local path, returns it.
    #[getter]
    pub fn local(&self) -> PyResult<Option<PathBuf>> {
        Ok(self.inner()?.local())
    }

    /// Returns a URL that is the equivalent of a "base directory" for the URL.
    ///
    /// The base URL will normally *not* have the query and fragment of this URL.
    ///
    /// Note that the base might not be readable, e.g. you would not be able to call
    /// [open](PyUrl::open) on it if it is a filesystem directory.
    #[getter]
    pub fn base(&self) -> PyResult<Option<Self>> {
        Ok(self.inner()?.base().map(|url| url.into()))
    }

    /// Parses the argument as a path relative to the URL. That means that this
    /// URL is treated as a "base directory" (see [base](PyUrl::base)). The argument
    /// supports ".." and ".", with the returned URL path always being absolute.
    ///
    /// The relative URL will normally *not* have the query and fragment of this URL.
    pub fn relative(&self, path: &str) -> PyResult<Self> {
        Ok(self.inner()?.relative(path).into())
    }

    /// Ensures that the URL conforms with the expectations of its functions. If
    /// successful, this function may change the URL appropriately, e.g. a relative
    /// path would be turned into an absolute path.
    ///
    /// This includes the expectation that [open](PyUrl::open) would minimally succeed,
    /// e.g. that the file exists or that the network endpoint is responsive. It does
    /// not otherwise guarantee that reading would be successful.
    pub fn conform(&mut self) -> PyResult<()> {
        self.inner()?.conform().into_py()
    }

    /// Opens the URL for reading by providing a [PyUrlReader].
    ///
    /// Note that for some URLs it may involve lengthy operations, e.g. cloning a
    /// remote repository, download a file, and/or unpacking an archive.
    ///
    /// Thus, an effort is made to not repeat these lengthy operations by caching
    /// relevant state via the URL's [PyUrlContext](super::PyUrlContext). For example, when accessing a "git:"
    /// URL on a remote repository that repository will be cloned locally only if it's
    /// the first time the repository has been referred to for the [UrlContext].
    /// Subsequent [open](PyUrl::open) calls for URLs that refer to the same git
    /// repository will reuse the existing clone.
    ///
    /// An effect of this optimization is that you might not be reading the most
    /// recent version of the resource the URL points to. If that is undesirable,
    /// call [reset](super::cache::UrlCache::reset) on the [UrlContext] cache.
    pub fn open(&self) -> PyResult<PyUrlReader> {
        self.inner()?.open().map(|reader| reader.into()).into_py()
    }

    /// Compare.
    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        Ok(op.matches(self.__str__()?.cmp(&other.__str__()?)))
    }

    /// Hash.
    pub fn __hash__(&self) -> PyResult<u64> {
        let mut hasher = DefaultHasher::new();
        self.__str__()?.hash(&mut hasher);
        Ok(hasher.finish())
    }

    /// As string.
    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.inner()?.to_string())
    }
}

impl PyUrl {
    fn inner(&self) -> PyResult<MutexGuard<'_, UrlRef>> {
        self.inner.lock().into_thread_problem().into_py()
    }

    /// Clone as reference.
    pub fn cloned(&self) -> PyResult<UrlRef> {
        Ok(self.inner()?.cloned())
    }
}

impl From<Mutex<UrlRef>> for PyUrl {
    fn from(inner: Mutex<UrlRef>) -> Self {
        Self { inner }
    }
}

impl From<UrlRef> for PyUrl {
    fn from(inner: UrlRef) -> Self {
        Mutex::new(inner).into()
    }
}
