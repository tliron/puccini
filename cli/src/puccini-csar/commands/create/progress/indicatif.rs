use super::super::{super::root::*, command::*};

use {indicatif::*, puccini_csar::creator::*, std::io};

impl Create {
    pub fn indicatif_progress_bar(root: &Root) -> ReadTrackerRef {
        // Note: the default target is stderr, which is what we want
        let progress_bar = ProgressBar::no_length();

        progress_bar.set_prefix("Compressing");

        progress_bar.set_style(
            ProgressStyle::with_template(if root.colorize.colorize() {
                "{prefix} {binary_total_bytes:.magenta} {bar:40.green} {spinner:.yellow} {bytes_per_sec:.magenta} ~{eta:.yellow} left"
            } else {
                "{prefix} {binary_total_bytes} {bar:40} {spinner} {bytes_per_sec} ~{eta} left"
            })
            .expect("ProgressStyle::with_template")
            .progress_chars("█▓▒")
            .tick_chars("▁▂▃▄▅▆▇█▇▆▅▄▃▂▁"),
        );

        Box::new(ProgressBarWrapper(progress_bar))
    }
}

//
// ProgressBarWrapper
//

// We need a wrapper because of the orphan rule
struct ProgressBarWrapper(ProgressBar);

impl ReadTracker for ProgressBarWrapper {
    fn initialize(&self, size: u64) {
        self.0.set_length(size);
    }

    fn finish(&self, _completed: bool) {}

    fn track<'read>(&self, reader: &'read mut dyn io::Read) -> Box<dyn io::Read + 'read> {
        Box::new(self.0.wrap_read(reader))
    }
}
