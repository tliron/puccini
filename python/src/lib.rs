mod compris;
mod csar;
mod floria;
mod problemo;
mod readurl;
mod tosca;

use pyo3::prelude::*;

#[pymodule]
pub mod puccini {
    use pyo3::prelude::*;

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
    pub mod readurl {
        use crate::readurl::*;

        #[pymodule_export]
        use PyUrlContext;

        #[pymodule_export]
        use PyUrl;

        #[pymodule_export]
        use PyUrlReader;

        #[pymodule_export]
        use py_format_archive_url;
    }

    #[pymodule]
    pub mod problemo {
        use crate::problemo::*;

        #[pymodule_export]
        use PyProblem;

        #[pymodule_export]
        use PyProblems;
    }

    #[pymodule]
    pub mod compris {
        use crate::compris::*;

        #[pymodule_export]
        use PyAnnotations;
    }
}
