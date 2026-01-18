use {pyo3::prelude::*, std::iter::*};

/// Add module.
///
/// Only needed for submodules.
///
/// See: https://github.com/PyO3/pyo3/issues/759#issuecomment-2282197848
#[allow(unused)]
pub fn add_submodule(module: &Bound<'_, PyModule>, parent_name: &[&str]) -> PyResult<()> {
    let name = vec_and(parent_name.into_iter().map(|s| *s), module.name()?.to_str()?).join(".");
    module.py().import("sys")?.getattr("modules")?.set_item(name, module)
}

fn vec_and<ItemT, IteratorT>(iterator: IteratorT, last: ItemT) -> Vec<ItemT>
where
    IteratorT: IntoIterator<Item = ItemT>,
{
    iterator.into_iter().chain(once(last)).collect()
}
