use super::Record;
use pgrx::{
    pg_sys::{Name, Oid},
    prelude::*,
};

/// Convenience wrapper for the configurable parts of an entry in the pg_database table.
/// Struct fields are the only configurable or inherited columns needed for pg_branch.
/// All other columns are hard-coded when converted to a Record for insertion.
// FIXME: pull more of these fields from the template instead of hard-coding during Into<Record>
pub struct PgDatabaseEntry {
    oid: Oid,
    name: Name,
    dba: Oid,
    tablespace: Oid,
}

impl PgDatabaseEntry {
    pub fn new(oid: Oid, name: Name, dba: Oid, tablespace: Oid) -> Self {
        Self {
            oid,
            name,
            dba,
            tablespace,
        }
    }
}

impl From<PgDatabaseEntry> for Record {
    fn from(entry: PgDatabaseEntry) -> Self {
        // these record fields match the order of columns in pg_database
        vec![
            entry.oid.into_datum(),
            Some(pg_sys::Datum::from(entry.name)),
            entry.dba.into_datum(),
            6.into_datum(),
            'c'.into_datum(),
            false.into_datum(),
            true.into_datum(),
            (-1).into_datum(),
            716.into_datum(),
            1.into_datum(),
            entry.tablespace.into_datum(),
            "C.UTF-8".into_datum(),
            "C.UTF-8".into_datum(),
            None,
            None,
            None,
        ]
    }
}
