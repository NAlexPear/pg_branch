use pgrx::pg_sys::Datum;

pub mod pg_database;

/// Type alias for a Record that can be inserted into a database
/// when paired with a TupleDescriptor via PgHeapTuple
pub type Record = Vec<Option<Datum>>;
