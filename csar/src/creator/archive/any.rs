use {kutil::std::any::*, std::io};

//
// AnyWriter
//

/// [io::Write] that can be converted to [Any](std::any::Any).
pub trait AnyWriter: IntoAny + io::Write {}

/// Common reference type for [AnyWriter].
pub type AnyWriterRef = Box<dyn AnyWriter>;

//
// AnySeekWriter
//

/// [io::Seek] + [io::Write] that can be converted to [Any](std::any::Any).
pub trait AnySeekWriter: IntoAny + io::Seek + io::Write {}

/// Common reference type for [AnySeekWriter].
pub type AnySeekWriterRef = Box<dyn AnySeekWriter>;
