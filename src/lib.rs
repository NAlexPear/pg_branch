use pgrx::prelude::*;

mod btrfs;
mod hooks;

pgrx::pg_module_magic!();

// PREREQ: configure mounted thumb drive as BTRFS
// PREREQ: configure BTRFS subvolume in mounted drive as the postgres data directory
// PREREQ: configure mounted drive as the postgres data directory
// PREREQ: configure each segment data subdirectory as a nested subvolume

#[pg_extern]
fn fork(_target_port: i32) {
    // TODO: btrfsutils create subvolume snapshot
    // TODO: recursively create subvolume snapshot of segment data subvolumes in the new fork
    // TODO: remove fork's postmaster.pid
    // TODO: set a new port in postgres.conf
    // TODO: create a recovery.conf setting restore_command to /bin/false
    // TODO: start the new forked cluster
}

#[pg_extern]
fn branch(target: &str, template: Option<&str>) {
    notice!(
        "target: {}, template: {}",
        target,
        template.unwrap_or("template0")
    );
    // TODO: get the OID of the template database from pg_database
    // TODO: verify that the data directory of the template exist and is a BTRFS subvolume
    // TODO: generate a new OID for the new database (via INSERT of the pg_database table?)
    // TODO: create a snapshot of the template database using the new OID
    // TODO: emulate whatever else createdb does to make sure that the new database is usable
}

#[pg_guard]
pub unsafe extern "C" fn _PG_init() {
    hooks::init()
}

#[pg_guard]
pub extern "C" fn _PG_fini() {
    // noop
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
