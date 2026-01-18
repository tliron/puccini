use super::problem::*;

use {problemo::*, pyo3::prelude::*};

//
// IntoPyResult
//

/// Map [Err] into Python error.
#[allow(unused)]
pub trait IntoPyResult<OkT> {
    /// Map [Err] into Python error.
    fn into_py(self) -> PyResult<OkT>;
}

impl<ResultT, OkT> IntoPyResult<OkT> for ResultT
where
    ResultT: IntoProblemResult<OkT>,
{
    #[track_caller]
    fn into_py(self) -> PyResult<OkT> {
        match self.into_problem() {
            Ok(ok) => Ok(ok),
            Err(problem) => Err(problem.into_py()),
        }
    }
}
