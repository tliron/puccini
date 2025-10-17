use super::{super::errors::*, archive::*, creator::*};

use {duplicate::*, std::io};

impl CsarCreator {
    #[duplicate_item(
        function                 ArchiveT                Where;
        [create_for_writer]      [ArchiveWithWriter]     [io::Write];
        [create_for_seek_writer] [ArchiveWithSeekWriter] [io::Seek + io::Write];
    )]
    /// Create an [Archive] for a writer.
    pub fn function<WriteT>(&mut self, writer: WriteT) -> Result<ArchiveT, CsarError>
    where
        WriteT: 'static + Where,
    {
        let mut archive = match self.format {
            Some(format) => ArchiveT::new_for(writer.into_any_writer(), format, self.compression)?,
            None => return Err(CsarError::Missing("must specify format".into())),
        };

        // Add meta first

        let meta = self.into_meta().stringify(self.max_columns)?;
        archive.add_string("TOSCA.meta", &meta, self.compression, None)?;

        Ok(archive)
    }
}
