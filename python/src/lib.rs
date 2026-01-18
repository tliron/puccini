use pyo3::prelude::*;

#[pymodule]
pub mod puccini {
    use {
        ::compris::pyo3::{problemo::*, *},
        kutil::pyo3::*,
        pyo3::prelude::*,
    };

    #[pymodule_init]
    fn init(module: &Bound<'_, PyModule>) -> PyResult<()> {
        register_submodules(module, "puccini", &["tosca", "csar", "floria", "readurl", "compris"])
    }

    #[pymodule_export]
    use PyAnnotations;

    #[pymodule_export]
    use PyProblem;

    #[pymodule_export]
    use PyProblems;

    #[pymodule]
    #[pyo3(module = "puccini.tosca")]
    pub mod tosca {
        use puccini_tosca::pyo3::*;

        #[pymodule_export]
        use py_compile_service_template;
    }

    #[pymodule]
    #[pyo3(module = "puccini.csar")]
    pub mod csar {
        use puccini_csar::pyo3::*;

        #[pymodule_export]
        use PyToscaMeta;

        #[pymodule_export]
        use PyToscaMetaBlock;

        #[pymodule_export]
        use PyCsarCreator;

        #[pymodule_export]
        use PyCreatedCsar;

        #[pymodule_export]
        use PyCsarArchiveWriter;
    }

    #[pymodule]
    pub mod floria {
        use floria::pyo3::*;

        #[pymodule_export]
        use PyStore;

        #[pymodule_export]
        use PyVertexTemplate;

        #[pymodule_export]
        use PyEdgeTemplate;

        #[pymodule_export]
        use PyVertex;

        #[pymodule_export]
        use PyEdge;

        #[pymodule_export]
        use PyClass;

        #[pymodule_export]
        use PyPlugin;

        #[pymodule_export]
        use PyProperty;

        #[pymodule_export]
        use PyID;

        #[pymodule_export]
        use PyFunctionName;
    }

    #[pymodule]
    pub mod readurl {
        use read_url::pyo3::*;

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
    pub mod compris {
        use compris::pyo3::*;

        #[pymodule_export]
        use PyAnnotations;
    }
}
