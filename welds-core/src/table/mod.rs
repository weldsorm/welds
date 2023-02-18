pub trait TableInfo {
    /// the unique name (schema + tablename) that identities this database object
    fn identifier() -> &'static str;

    fn columns() -> &'static [&'static str];
}
