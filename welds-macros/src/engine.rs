#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Engine {
    Postgres,
    Mysql,
    Sqlite,
    Mssql,
}

impl Engine {
    pub(crate) fn parse(meta: &syn::Meta) -> Option<Engine> {
        if meta.path().is_ident("Postgres") {
            return Some(Engine::Postgres);
        }
        if meta.path().is_ident("Mysql") {
            return Some(Engine::Mysql);
        }
        if meta.path().is_ident("Mssql") {
            return Some(Engine::Mssql);
        }
        if meta.path().is_ident("sqlite") {
            return Some(Engine::Sqlite);
        }
        None
    }

    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Postgres => "Postgres",
            Self::Mysql => "MySql",
            Self::Mssql => "Mssql",
            Self::Sqlite => "Sqlite",
        }
    }
}

pub(crate) const ALL: &[Engine] = &[
    #[cfg(feature = "postgres")]
    Engine::Postgres,
    #[cfg(feature = "mysql")]
    Engine::Mysql,
    #[cfg(feature = "mssql")]
    Engine::Mssql,
    #[cfg(feature = "sqlite")]
    Engine::Sqlite,
];
