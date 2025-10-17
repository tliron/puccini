use std::io;

//
// ReadTracker
//

/// Common reference type for [ReadTracker].
pub type ReadTrackerRef = Box<dyn ReadTracker>;

/// Read tracker.
///
/// This trait is `dyn`-compatible.
pub trait ReadTracker {
    /// Initialize with size.
    fn initialize(&self, size: u64);

    /// Finish tracking.
    fn finish(&self, completed: bool);

    /// Track.
    fn track<'read>(&self, reader: &'read mut dyn io::Read) -> Box<dyn io::Read + 'read>;
}

//
// ReadTrackerChain
//

/// Chain of [ReadTracker] implementations.
pub struct ReadTrackerChain {
    /// Read trackers.
    pub trackers: Vec<ReadTrackerRef>,
}

impl ReadTrackerChain {
    /// Create a new [ReadTrackerChain].
    pub fn new(inner: Vec<ReadTrackerRef>) -> Self {
        Self { trackers: inner }
    }
}

impl ReadTracker for ReadTrackerChain {
    fn initialize(&self, size: u64) {
        for tracker in &self.trackers {
            tracker.initialize(size);
        }
    }

    fn finish(&self, completed: bool) {
        for tracker in self.trackers.iter().rev() {
            tracker.finish(completed);
        }
    }

    fn track<'read>(&self, reader: &'read mut dyn io::Read) -> Box<dyn io::Read + 'read> {
        let mut reader: Box<dyn io::Read + 'read> = Box::new(reader);
        for tracker in &self.trackers {
            reader = tracker.track(Box::leak(reader));
        }
        reader
    }
}
