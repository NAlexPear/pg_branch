pub use btrfs::Btrfs;
use std::path::PathBuf;

mod btrfs;

/// The core of any file system's ability to support branching databases
/// is its capacity for atomic snapshots. The Branching trait encapsulates
/// that behavior, and can be implemented by any copy-on-write file system.
pub trait Branching {
    /// Create a snapshot of one "subvolume" (or whatever the equivalent term might be)
    /// This method is expected to panic if something goes wrong.
    fn create_snapshot(source: PathBuf, destination: PathBuf);
}
