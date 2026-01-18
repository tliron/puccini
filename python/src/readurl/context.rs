use super::{super::problemo::*, url::*};

use {pyo3::prelude::*, read_url::*};

//
// PyContext
//

/// URL context.
#[pyclass(name = "Context", frozen)]
pub struct PyUrlContext {
    /// Inner URL context reference.
    pub inner: UrlContextRef,
}

#[pymethods]
impl PyUrlContext {
    /// Constructor.
    #[new]
    pub fn new() -> Self {
        UrlContext::new().into()
    }

    /// Base URLs.
    #[getter]
    pub fn base_urls(&self) -> PyResult<Vec<PyUrl>> {
        Ok(self.inner.clone_base_urls().into_iter().map(|url| url.cloned().into()).collect())
    }

    /// Return a child context with different base URLs.
    ///
    /// The child context shares everything else with the parent.
    pub fn with_base_urls(&self, base_urls: Vec<Bound<'_, PyAny>>) -> PyResult<Self> {
        let mut base_urls_ = Vec::<UrlRef>::with_capacity(base_urls.len());

        for base_url in base_urls {
            base_urls_.push(self.to_url_ref(base_url)?)
        }

        Ok(self.inner.clone().with_base_urls(base_urls_).into())
    }

    /// Return a child context with a different cache base directory.
    ///
    /// The child context shares everything else with the parent.
    pub fn with_cache(&self, cache_base_directory: Option<String>) -> Self {
        self.inner.with_cache(cache_base_directory.map(|path| path.into())).into()
    }

    /// Override a URL.
    pub fn override_url(&self, from_url: String, to_url: String) -> PyResult<Option<String>> {
        self.inner.override_url(from_url, to_url).into_py()
    }

    /// Remove a URL override.
    pub fn remove_url_override(&self, from_url: String) -> PyResult<Option<String>> {
        self.inner.remove_url_override(&from_url).into_py()
    }

    /// Override a global URL.
    #[staticmethod]
    pub fn override_global_url(from_url: String, to_url: String) -> PyResult<Option<String>> {
        UrlContext::override_global_url(from_url, to_url).into_py()
    }

    /// Remove a global URL override.
    #[staticmethod]
    pub fn remove_global_url_override(from_url: String) -> PyResult<Option<String>> {
        UrlContext::remove_global_url_override(&from_url).into_py()
    }

    /// Get a URL override.
    ///
    /// Tries the context's overrides first, the global overrides next.
    pub fn get_url_override(&self, from_url: String) -> PyResult<Option<String>> {
        self.inner.get_url_override(&from_url).into_py()
    }

    /// Get a URL's override or itself.
    ///
    /// Tries the context's overrides first, the global overrides next.
    pub fn get_url_or_override(&self, from_url: String) -> PyResult<String> {
        self.inner.get_url_or_override(from_url).into_py()
    }

    /// Parses the argument as either an absolute URL or a path relative to
    /// one of the context's base URls. Relative paths support ".." and ".".
    ///
    /// The returned URL will always have had [PyUrl::conform] called on it, so
    /// there is no need to call it again.
    ///
    /// Relative paths are tested against the base URLs argument in order. The
    /// first valid URL will be returned and the remaining bases will be ignored.
    /// Note that bases can be any of any URL type.
    ///
    /// If you are expecting either a URL or a file path, consider
    /// [url_or_file_path](PyUrlContext::url_or_file_path).
    pub fn url(&self, url_representation: &str) -> PyResult<PyUrl> {
        self.inner.url(url_representation).map(|url| url.into()).into_py()
    }

    /// Parses the argument as an absolute URL, or an absolute file path, or a
    /// path relative to one of the context's base URLs. Relative paths support
    /// ".." and ".".
    ///
    /// The returned URL will always have had [PyUrl::conform] called on it, so
    /// there is no need to call it again.
    ///
    /// Relative paths are tested against the base URLs argument in order. The
    /// first valid URL will be returned and the remaining bases will be ignored.
    /// Note that bases can be any of any URL type.
    ///
    /// On Windows note a rare edge case: If there happens to be a drive that has the
    /// same name as a supported URL scheme (e.g. "http") then callers would have to
    /// provide a full file URL, e.g. instead of "http:\Dir\file" provide
    /// "file:///http:/Dir/file". Otherwise it would be parsed as a URL of that scheme.
    /// rather than a file path.
    pub fn url_or_file_path(&self, url_or_file_path_representation: &str) -> PyResult<PyUrl> {
        self.inner.url_or_file_path(url_or_file_path_representation).map(|url| url.into()).into_py()
    }

    /// Parses the argument as an absolute URL.
    ///
    /// Make sure to call [PyUrl::conform] before calling [PyUrl::open].
    ///
    /// To support relative URLs, see [url](PyUrlContext::url).
    ///
    /// If you are expecting either a URL or a file path, consider
    /// [absolute_url_or_file_path](PyUrlContext::absolute_url_or_file_path).
    pub fn absolute_url(&self, url_representation: &str) -> PyResult<PyUrl> {
        self.inner.absolute_url(url_representation).map(|url| url.into()).into_py()
    }

    /// Parses the argument as either an absolute URL or an absolute file path.
    ///
    /// Make sure to call [PyUrl::conform] before calling [PyUrl::open].
    ///
    /// Internally, attempts to parse the URL via
    /// [absolute_url](PyUrlContext::absolute_url) and if that fails treats
    /// the URL as an absolute file path and returns a `FileUrl`.
    ///
    /// To support relative URLs and relative file paths, see
    /// [url_or_file_path](PyUrlContext::url_or_file_path).
    ///
    /// On Windows note a rare edge case: If there happens to be a drive that has the
    /// same name as a supported URL scheme (e.g. "http") then callers would have to
    /// provide a full file URL, e.g. instead of "http:\Dir\file" provide
    /// "file:///http:/Dir/file". Otherwise it would be parsed as a URL of that scheme.
    /// rather than a file path.
    pub fn absolute_url_or_file_path(&self, url_or_file_path_representation: &str) -> PyResult<PyUrl> {
        self.inner.absolute_url_or_file_path(url_or_file_path_representation).map(|url| url.into()).into_py()
    }

    /// A valid `FileUrl` for the current working directory.
    pub fn working_dir_url(&self) -> PyResult<PyUrl> {
        self.inner.working_dir_url().map(|url| url.into()).into_py()
    }
}

impl PyUrlContext {
    fn to_url_ref(&self, url: Bound<'_, PyAny>) -> PyResult<UrlRef> {
        Ok(match url.cast::<PyUrl>() {
            Ok(url) => url.borrow().cloned()?,
            Err(_) => self.inner.url_or_file_path(url.str().into_py()?.to_str().into_py()?).into_py()?,
        })
    }
}

impl From<UrlContextRef> for PyUrlContext {
    fn from(inner: UrlContextRef) -> Self {
        Self { inner }
    }
}
