use super::any::*;

use {
    kutil::std::any::*,
    std::{any::*, io},
};

//
// AnyWriterWrapper
//

/// Wrapper that implements [AnyWriter] and [AnySeekWriter].
pub struct AnyWriterWrapper<WriteT> {
    /// Inner.
    pub inner: Option<WriteT>,
}

impl<WriteT> AnyWriterWrapper<WriteT> {
    /// Constructor.
    pub fn new(inner: WriteT) -> Box<Self> {
        Box::new(Self { inner: Some(inner) })
    }
}

impl<WriteT> IntoAny for AnyWriterWrapper<WriteT>
where
    WriteT: Any,
{
    fn into_any(&mut self) -> Box<dyn Any> {
        Box::new(self.inner.take().unwrap())
    }
}

impl<WriteT> AnyWriter for AnyWriterWrapper<WriteT> where WriteT: Any + io::Write {}

impl<WriteT> AnySeekWriter for AnyWriterWrapper<WriteT> where WriteT: Any + io::Seek + io::Write {}

impl<WriteT> io::Seek for AnyWriterWrapper<WriteT>
where
    WriteT: io::Seek,
{
    fn seek(&mut self, from: io::SeekFrom) -> io::Result<u64> {
        self.inner.as_mut().unwrap().seek(from)
    }
}

impl<WriteT> io::Write for AnyWriterWrapper<WriteT>
where
    WriteT: io::Write,
{
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.inner.as_mut().unwrap().write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.as_mut().unwrap().flush()
    }
}

//
// IntoAnyWriter
//

/// Convert into an [AnyWriter] and [AnySeekWriter] implementation.
pub trait IntoAnyWriter<WriteT> {
    /// Convert into an [AnyWriter] and [AnySeekWriter] implementation.
    fn into_any_writer(self) -> Box<AnyWriterWrapper<WriteT>>;
}

impl<WriteT> IntoAnyWriter<WriteT> for WriteT
where
    WriteT: Any + io::Write,
{
    fn into_any_writer(self) -> Box<AnyWriterWrapper<WriteT>> {
        AnyWriterWrapper::new(self)
    }
}
