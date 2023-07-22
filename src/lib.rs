#![cfg_attr(not(test), deny(unused_crate_dependencies))]

use pgrx::{pg_sys::Oid, prelude::*, PgRelation};
use std::{ffi::c_char, os::unix::ffi::OsStrExt, path::Path};

mod btrfs;
mod hooks;

pgrx::pg_module_magic!();

#[pg_extern]
fn branch(target: &str, template: Option<&str>) {
    let template = template.unwrap_or("template1");

    // check that the target database doesn't already exist
    let no_duplicate = Spi::connect(|client| {
        client
            .select(
                "select oid from pg_database where datname = $1",
                Some(1),
                Some(vec![(
                    PgOid::BuiltIn(PgBuiltInOids::TEXTOID),
                    target.into_datum(),
                )]),
            )
            .map(|result| result.is_empty())
    })
    .expect("Error querying pg_database table");

    if !no_duplicate {
        error!(r#"database "{}" already exists"#, target);
    }

    // get the data directory and OID of the template database from pg_database
    let segment_path_components: (Option<String>, Option<Oid>) = Spi::connect(|client| {
        let data_directory = client
            .select(
                "select setting from pg_settings where name = 'data_directory'",
                Some(1),
                None,
            )?
            .first()
            .get_one()?;

        let template_oid = client
            .select(
                "select oid from pg_database where datname = $1",
                Some(1),
                Some(vec![(
                    PgOid::BuiltIn(PgBuiltInOids::TEXTOID),
                    template.into_datum(),
                )]),
            )?
            .first()
            .get_one()?;

        Ok::<_, spi::Error>((data_directory, template_oid))
    })
    .expect("Error querying pg_database table");

    // generate the path to the segment data of the template database
    let template_data_path = match segment_path_components {
        (None, _) => FATAL!("no data_directory found"),
        (_, None) => error!(
            "template {} does not exist in the pg_database table",
            template
        ),
        (Some(data_directory), Some(template_oid)) => Path::new(&data_directory)
            .join("base")
            .join(template_oid.as_u32().to_string()),
    };

    // generate a new OID for the new database (via cluster oid generator)
    let target_oid = unsafe { pg_sys::GetNewObjectId() };

    let target_data_path = template_data_path
        .parent()
        .expect("Invalid template data path")
        .join(target_oid.as_u32().to_string());

    // create a snapshot of the template database using the new OID
    let exit_code = unsafe {
        let template_path = std::ffi::CString::new(template_data_path.as_os_str().as_bytes())
            .expect("Invalid template data path");

        let target_path = std::ffi::CString::new(target_data_path.as_os_str().as_bytes())
            .expect("Invalid target data path");

        btrfs::btrfs_util_create_snapshot(
            template_path.as_ptr(),
            target_path.as_ptr(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if exit_code > 0 {
        panic!("Failed to create a snapshot of database {}", template);
    }

    // update the pg_database catalog table with the new database information
    // FIXME: pull most of these fields from the template instead of hard-coding them
    let mut target_name_data = [0 as c_char; 64];
    for (left, right) in target_name_data.iter_mut().zip(
        std::ffi::CString::new(target)
            .expect("Invalid target")
            .as_bytes_with_nul(),
    ) {
        *left = *right as i8;
    }
    let mut target_name = pg_sys::nameData {
        data: target_name_data,
    };
    let pg_database_record = vec![
        target_oid.into_datum(),
        Some(pg_sys::Datum::from(
            &mut target_name as *mut pg_sys::nameData as pg_sys::Name,
        )),
        10.into_datum(),
        6.into_datum(),
        'c'.into_datum(),
        false.into_datum(),
        true.into_datum(),
        (-1).into_datum(),
        716.into_datum(),
        1.into_datum(),
        1663.into_datum(),
        "C.UTF-8".into_datum(),
        "C.UTF-8".into_datum(),
        None,
        None,
        None,
    ];
    let pg_database = PgRelation::open_with_name_and_share_lock("pg_database")
        .expect("Relation pg_database not found");
    let pg_database_tuple_descriptor = pg_database.tuple_desc();

    unsafe {
        let tuple = PgHeapTuple::from_datums(pg_database_tuple_descriptor, pg_database_record)
            .expect("Failed to create the new heap tuple");
        pg_sys::CatalogTupleInsert(pg_database.as_ptr(), tuple.into_pg());
    };
}

#[pg_guard]
#[allow(clippy::missing_safety_doc)]
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
