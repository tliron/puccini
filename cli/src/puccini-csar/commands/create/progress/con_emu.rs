use super::super::command::*;

use {
    kutil::{cli::con_emu::*, std::exit::*},
    puccini_csar::creator::*,
    std::{io, sync::*},
};

impl Create {
    pub fn con_emu_osc_progress_state() -> ReadTrackerRef {
        // Make sure to remove on exit
        on_exit(|| ConEmuProgressState::Remove.write(0));

        Box::new(ProgressStateWrapper::default())
    }
}

//
// ConEmuProgressStateTracker
//

type ConEmuProgressStateTrackerRef = Arc<ConEmuProgressStateTracker>;

// We need a wrapper because of the orphan rule
#[derive(Default)]
struct ProgressStateWrapper(ConEmuProgressStateTrackerRef);

impl ReadTracker for ProgressStateWrapper {
    fn initialize(&self, size: u64) {
        self.0.start(size);
    }

    fn finish(&self, _completed: bool) {
        ConEmuProgressState::Remove.write(0);
    }

    fn track<'read>(&self, reader: &'read mut dyn io::Read) -> Box<dyn io::Read + 'read> {
        Box::new(ReaderWrapper::new(reader, self.0.clone()))
    }
}

//
// ReaderWrapper
//

struct ReaderWrapper<'read> {
    reader: &'read mut dyn io::Read,
    progress_state: ConEmuProgressStateTrackerRef,
}

impl<'read> ReaderWrapper<'read> {
    fn new(reader: &'read mut dyn io::Read, progress_state: ConEmuProgressStateTrackerRef) -> Self {
        Self { reader, progress_state }
    }
}

impl<'read> io::Read for ReaderWrapper<'read> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self.reader.read(buffer) {
            Ok(count) => {
                self.progress_state.add(count as u64);
                Ok(count)
            }

            Err(error) => {
                ConEmuProgressState::Error.write(0);
                Err(error)
            }
        }
    }
}
