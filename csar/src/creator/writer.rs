use super::{super::errors::*, archive::*, creator::*};

use {
    duplicate::*,
    kutil::io::writer::*,
    problemo::{common::*, *},
    std::io,
};

impl CsarCreator {
    #[duplicate_item(
        writer          ArchiveT              WriteTraits;
        [stream_writer] [ArchiveStreamWriter] [io::Write];
        [seek_writer]   [ArchiveSeekWriter]   [io::Seek + io::Write];
    )]
    /// Create an [Archive] writer.
    pub fn writer<WriteT>(&self, writer: WriteT) -> Result<ArchiveT, Problem>
    where
        WriteT: 'static + WriteTraits + Send,
    {
        let mut archive = match self.format {
            Some(format) => ArchiveT::new_for(writer.into_any_writer(), format, self.compression_level)?,
            None => return Err(MissingError::as_problem("must specify format").via(CsarError)),
        };

        // Add TOSCA.meta first

        let tosca_meta = self.to_tosca_meta().stringify(self.max_columns)?;
        archive.add_string("TOSCA.meta", &tosca_meta, self.compression_level, None)?;

        Ok(archive)
    }

    #[duplicate_item(
        into_writer          ArchiveT              WriteTraits;
        [into_stream_writer] [ArchiveStreamWriter] [io::Write];
        [into_seek_writer]   [ArchiveSeekWriter]   [io::Seek + io::Write];
    )]
    /// Create an [Archive] writer.
    pub fn into_writer<WriteT>(self, writer: WriteT) -> Result<ArchiveT, Problem>
    where
        WriteT: 'static + WriteTraits + Send,
    {
        let mut archive = match self.format {
            Some(format) => ArchiveT::new_for(writer.into_any_writer(), format, self.compression_level)?,
            None => return Err(MissingError::as_problem("must specify format").via(CsarError)),
        };

        // Add TOSCA.meta first

        let compression_level = self.compression_level;
        let max_columns = self.max_columns;
        let tosca_meta = self.into_tosca_meta().stringify(max_columns)?;
        archive.add_string("TOSCA.meta", &tosca_meta, compression_level, None)?;

        Ok(archive)
    }
}
