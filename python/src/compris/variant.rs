use {
    compris::{annotate::*, normal::*},
    pyo3::{prelude::*, types::*},
};

//
// IntoPy
//

/// Into Python.
pub trait IntoPy {
    /// Into Python.
    fn into_py<'py>(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>>;
}

impl IntoPy for Variant<WithoutAnnotations> {
    fn into_py<'py>(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Ok(match self {
            Variant::Undefined => PyNotImplemented::get(py).to_owned().into_any(),
            Variant::Null(_) => PyNone::get(py).to_owned().into_any(),
            Variant::Integer(integer) => PyInt::new(py, integer.inner).into_any(),
            Variant::UnsignedInteger(unsigned_integer) => PyInt::new(py, unsigned_integer.inner).into_any(),
            Variant::Float(float) => PyFloat::new(py, float.into()).into_any(),
            Variant::Boolean(boolean) => PyBool::new(py, boolean.into()).to_owned().into_any(),
            Variant::Text(text) => PyString::new(py, text.as_ref()).into_any(),
            Variant::Blob(blob) => PyBytes::new(py, blob.as_ref()).into_any(),

            Variant::List(list) => {
                let mut items = Vec::with_capacity(list.inner.len());
                for item in list {
                    items.push(item.into_py(py)?);
                }
                PyList::new(py, items)?.into_any()
            }

            Variant::Map(map) => {
                let dict = PyDict::new(py);
                for (key, value) in map {
                    if key.is_collection() {
                        dict.set_item(key.into_py(py)?.repr()?, value.into_py(py)?)?;
                    } else {
                        dict.set_item(key.into_py(py)?, value.into_py(py)?)?;
                    };
                }
                dict.into_any()
            }
        })
    }
}
