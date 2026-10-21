use std::io;
use std::num::NonZeroUsize;

/// A snapshot of the estimated parallel capacity available to this program.
///
/// This does not describe the CPU model or necessarily the physical core count.
#[derive(Debug, Clone, Copy)]
pub struct CpuInfo {
    available_parallelism: NonZeroUsize,
}

impl CpuInfo {
    /// Detect CPU information from the running system.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the platform cannot report parallelism or
    /// the program lacks permission to query it.
    pub fn detect() -> io::Result<Self> {
        let available_parallelism = std::thread::available_parallelism()?;
        Ok(Self {
            available_parallelism,
        })
    }

    /// Estimated number of threads that can run in parallel.
    ///
    /// This is **not** necessarily the physical core count — see
    /// <https://doc.rust-lang.org/std/thread/fn.available_parallelism.html>.
    pub fn available_parallelism(&self) -> NonZeroUsize {
        self.available_parallelism
    }
}
