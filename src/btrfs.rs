#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Id of the root subvolume in a BTRFS filesystem.
pub const BTRFS_FS_TREE_OBJECTID: u64 = 5;
