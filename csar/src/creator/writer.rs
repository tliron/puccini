use super::{super::errors::*, archive::*, creator::*};

use {duplicate::*, kutil::io::writer::*, std::io};

impl CsarCreator {
    #[duplicate_item(
        create_for_writer        ArchiveT            Where;
        [create_for_writer]      [ArchiveWriter]     [io::Write];
        [create_for_seek_writer] [ArchiveSeekWriter] [io::Seek + io::Write];
    )]
    /// Create an [Archive] for a writer.
    pub fn create_for_writer<WriteT>(&mut self, writer: WriteT) -> Result<ArchiveT, CsarError>
    where
        WriteT: 'static + Where,
    {
        let mut archive = match self.format {
            Some(format) => ArchiveT::new_for(writer.into_any_writer(), format, self.compression_level)?,
            None => return Err(CsarError::Missing("must specify format".into())),
        };

        // Add TOSCA.meta first

        let tosca_meta = self.into_tosca_meta().stringify(self.max_columns)?;
        archive.add_string("TOSCA.meta", &tosca_meta, self.compression_level, None)?;

        Ok(archive)
    }
}
