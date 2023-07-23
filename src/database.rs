use pgrx::{
    pg_sys::{Datum, Name, Oid},
    prelude::*,
};
use std::{
    ffi::{c_char, CString},
    fmt::Display,
    path::{Path, PathBuf},
};

/// Type alias for a Record that can be inserted into a database
/// when paired with a TupleDescriptor via PgHeapTuple
type Record = Vec<Option<Datum>>;

/// Catalog entry in the pg_database table for tracking database info
pub struct Database {
    name: pg_sys::nameData,
    oid: Oid,
    pub dba: Oid,
    pub tablespace: Oid,
}

impl Database {
    /// Create a new database entry directly
    pub fn new<T>(name: T, oid: Oid, dba: Oid, tablespace: Oid) -> Self
    where
        T: Into<Vec<u8>> + Display + Copy,
    {
        // convert the name to a pg-compatible name data
        let mut name_data = [0 as c_char; 64];
        for (left, right) in name_data.iter_mut().zip(
            CString::new(name)
                .unwrap_or_else(|_| panic!("Invalid database name {name}"))
                .as_bytes_with_nul(),
        ) {
            *left = *right as i8;
        }
        let name = pg_sys::nameData { data: name_data };

        Self {
            name,
            oid,
            dba,
            tablespace,
        }
    }

    /// Query the catalog tables by the name of the database,
    /// panicking if the database doesn't exist or has an incomplete catalog entry
    pub fn find(name: &str) -> Self {
        // get the relevant database fields from pg_database
        let tuple = Spi::get_three_with_args(
            "select oid, datdba, dattablespace from pg_database where datname = $1",
            vec![(PgOid::BuiltIn(PgBuiltInOids::TEXTOID), name.into_datum())],
        )
        .expect("Error querying pg_database table");

        // validate the catalog tuple fields
        match tuple {
            (Some(oid), Some(dba), Some(tablespace)) => Self::new(name, oid, dba, tablespace),
            _ => panic!(r#"database "{name}" does not exist in the pg_database table"#),
        }
    }

    /// Get the directory where this database's data is stored
    pub fn data(&self) -> PathBuf {
        Spi::get_one("select setting from pg_settings where name = 'data_directory'")
            .expect("Error querying pg_settings table")
            .map(|data_directory: &str| {
                Path::new(data_directory)
                    .join("base")
                    .join(self.oid.as_u32().to_string())
            })
            .expect("No data_directory found!")
    }

    /// generate a Record that can be inserted into pg_database while
    /// continuing to reference data from this struct (hence the exclusive reference).
    pub fn as_record(&mut self) -> Record {
        // convert the name data to a pg-compatible "name" Datum
        let name = Datum::from(&mut self.name as *mut pg_sys::nameData as Name);

        // these record fields match the order of columns in pg_database
        vec![
            self.oid.into_datum(),
            Some(name),
            self.dba.into_datum(),
            6.into_datum(),
            'c'.into_datum(),
            false.into_datum(),
            true.into_datum(),
            (-1).into_datum(),
            716.into_datum(),
            1.into_datum(),
            self.tablespace.into_datum(),
            "C.UTF-8".into_datum(),
            "C.UTF-8".into_datum(),
            None,
            None,
            None,
        ]
    }
}
