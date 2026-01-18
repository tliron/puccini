use {
    compris::{annotate::*, normal::*},
    pyo3::{prelude::*, types::*},
};

//
// IntoPyAny
//

/// Into Python any.
pub trait IntoPyAny {
    /// Into Python any.
    fn into_py<'py>(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>>;
}

impl IntoPyAny for Variant<WithoutAnnotations> {
    /// Into [PyAny].
    fn into_py<'py>(self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        Ok(match self {
            Variant::Undefined | Variant::Null(_) => PyNone::get(py).to_owned().into_any(),
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
