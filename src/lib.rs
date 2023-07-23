#![cfg_attr(not(test), deny(unused_crate_dependencies))]

use database::Database;
use fs::{Branching, Btrfs};
use pgrx::{prelude::*, PgRelation};

mod database;
mod fs;
mod hooks;

pgrx::pg_module_magic!();

/// Create a branch of a template database using file system snapshots
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
        error!(r#"database "{target}" already exists"#);
    }

    // generate the template fields from the template name
    let template_fields = Database::find(template);
    let template_data_path = template_fields.data();

    // generate a new OID for the new database (via cluster oid generator)
    let target_oid = unsafe { pg_sys::GetNewObjectId() };

    let target_data_path = template_data_path
        .parent()
        .expect("Invalid template data path")
        .join(target_oid.as_u32().to_string());

    // create a snapshot of the template database using the new OID
    Btrfs::create_snapshot(template_data_path, target_data_path);

    // update the pg_database catalog table with the new database information
    let mut catalog_entry = Database::new(
        target,
        target_oid,
        template_fields.dba,
        template_fields.tablespace,
    );
    let pg_database = PgRelation::open_with_name_and_share_lock("pg_database")
        .expect("Relation pg_database not found");
    unsafe {
        let tuple = PgHeapTuple::from_datums(pg_database.tuple_desc(), catalog_entry.as_record())
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
