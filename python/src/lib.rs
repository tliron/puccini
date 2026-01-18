mod csar;
mod floria;
mod tosca;
mod url;
mod utils;

use pyo3::*;

#[pymodule]
pub mod puccini {
    use pyo3::*;

    #[pymodule]
    pub mod tosca {
        use crate::tosca::*;

        #[pymodule_export]
        use py_compile_service_template;
    }

    #[pymodule]
    pub mod csar {
        use crate::csar::*;

        #[pymodule_export]
        use py_create;
    }

    #[pymodule]
    pub mod floria {
        use crate::floria::*;

        #[pymodule_export]
        use PyStore;
    }

    #[pymodule]
    pub mod url {
        use crate::url::*;

        #[pymodule_export]
        use PyUrlContext;
    }
}
