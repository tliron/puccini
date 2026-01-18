use {
    compris::annotate::*,
    kutil::std::any::*,
    problemo::{common::*, *},
    std::any::*,
};

//
// DowncastRefChecked
//

/// As a concrete type reference.
pub trait DowncastRefChecked {
    /// As a concrete type reference.
    fn downcast_ref_checked<AnyT>(&self) -> Result<&AnyT, Problem>
    where
        AnyT: 'static;
}

impl<AsAnyRefT> DowncastRefChecked for AsAnyRefT
where
    AsAnyRefT: AsAnyRef,
{
    fn downcast_ref_checked<AnyT>(&self) -> Result<&AnyT, Problem>
    where
        AnyT: 'static,
    {
        self.downcast_ref().ok_or_else(|| {
            CastingError::as_problem(type_name::<AnyT>()).with(AnnotatedCauseEquality::new::<CastingError>())
        })
    }
}
