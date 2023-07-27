#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

use super::Branching;
use std::path::PathBuf;
use std::{ffi::c_char, os::unix::ffi::OsStrExt, path::Path};

/// Wrapper type used for implementing the Branching trait for the btrfs file system
pub struct Btrfs;

impl Branching for Btrfs {
    fn create_snapshot(source: PathBuf, destination: PathBuf) {
        let exit_code = unsafe {
            let source_path =
                std::ffi::CString::new(source.as_os_str().as_bytes()).expect("Invalid source path");

            let destination_path = std::ffi::CString::new(destination.as_os_str().as_bytes())
                .expect("Invalid destination path");

            btrfsutil_sys::btrfs_util_create_snapshot(
                source_path.as_ptr(),
                destination_path.as_ptr(),
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };

        if exit_code > 0 {
            panic!("Failed to create a snapshot");
        }
    }
}
