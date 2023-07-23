use pgrx::pg_sys::Oid;

/// Type alias for the tuple returned from pg_database for the template database.
pub type TemplateTuple = (Option<Oid>, Option<Oid>, Option<Oid>);

/// Relevant template table fields derived from the template tuple.
pub struct Template {
    oid: Oid,
    pub dba: Oid,
    pub tablespace: Oid,
}

impl Template {
    pub fn as_path_component(&self) -> String {
        self.oid.as_u32().to_string()
    }
}

impl TryFrom<TemplateTuple> for Template {
    type Error = ();

    fn try_from(tuple: TemplateTuple) -> Result<Self, Self::Error> {
        match tuple {
            (Some(oid), Some(dba), Some(tablespace)) => Ok(Self {
                oid,
                dba,
                tablespace,
            }),
            _ => Err(()),
        }
    }
}
