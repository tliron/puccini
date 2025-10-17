use super::wrong_type::*;

use {kutil::std::any::*, std::any::*};

//
// IntoAnyRefChecked
//

/// Convert into [Any] reference or return an error.
pub trait IntoAnyRefChecked {
    /// Convert into [Any] reference or return an error.
    fn into_any_ref_checked<AnyT, AnnotatedT>(
        &self,
        entity: &'static str,
        type_name: &'static str,
    ) -> Result<&AnyT, WrongTypeError<AnnotatedT>>
    where
        AnyT: Any,
        AnnotatedT: Default;
}

impl<IntoAnyRefT> IntoAnyRefChecked for IntoAnyRefT
where
    IntoAnyRefT: IntoAnyRef,
{
    fn into_any_ref_checked<AnyT, AnnotatedT>(
        &self,
        entity: &'static str,
        type_name: &'static str,
    ) -> Result<&AnyT, WrongTypeError<AnnotatedT>>
    where
        AnyT: Any,
        AnnotatedT: Default,
    {
        self.into_any_ref().ok_or_else(|| WrongTypeError::new(entity.into(), type_name.into(), Default::default()))
    }
}
