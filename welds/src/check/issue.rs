use crate::{
    detect::ColumnDef,
    model_traits::{Column, TableIdent},
};
use colored::Colorize;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diff {
    pub column: String,
    pub db_type: String,
    pub db_nullable: bool,
    pub welds_type: String,
    pub welds_nullable: bool,
    type_changed: bool,
}

impl Diff {
    /// returns true if the underlying DB_TYPE and MODEL_TYPE are no longer compatible
    pub fn type_changed(&self) -> bool {
        self.type_changed
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("The Column `")?;
        f.write_str(&self.column)?;
        f.write_str("` is has changed,")?;
        if self.type_changed {
            f.write_str(" db_type: ")?;
            f.write_str(&self.db_type)?;
            f.write_str(" welds_type: ")?;
            f.write_str(&self.welds_type)?;
        }
        if self.db_nullable != self.welds_nullable {
            f.write_str(" db_null: ")?;
            Display::fmt(&self.db_nullable, f)?;
            f.write_str(" welds_null: ")?;
            Display::fmt(&self.welds_nullable, f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Describes a potential problem or different between the welds struct and the database table
pub struct Issue {
    pub ident: TableIdent,
    pub level: Level,
    pub kind: Kind,
}

impl Display for Issue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.ident, f)?;
        f.write_str(" (")?;
        Display::fmt(&self.level, f)?;
        f.write_str("): ")?;
        Display::fmt(&self.kind, f)?;
        Ok(())
    }
}

impl Issue {
    pub(crate) fn missing_table(schemaname: Option<&str>, tablename: &str) -> Self {
        let ident = TableIdent {
            schema: schemaname.map(|x| x.to_string()),
            name: tablename.to_string(),
        };
        Issue {
            ident,
            level: Level::Critical,
            kind: Kind::MissingTable,
        }
    }

    pub(crate) fn struct_missing(
        schemaname: Option<&str>,
        tablename: &str,
        col: &ColumnDef,
    ) -> Self {
        let ident = TableIdent {
            schema: schemaname.map(|x| x.to_string()),
            name: tablename.to_string(),
        };
        Issue {
            ident,
            level: Level::Medium,
            kind: Kind::InDbNotModel(Missing {
                column: col.name.to_string(),
                ty: col.ty.to_string(),
                nullable: col.null,
            }),
        }
    }

    pub(crate) fn struct_added(schemaname: Option<&str>, tablename: &str, col: &Column) -> Self {
        let ident = TableIdent {
            schema: schemaname.map(|x| x.to_string()),
            name: tablename.to_string(),
        };
        Issue {
            ident,
            level: Level::Critical,
            kind: Kind::OnModelNotDb(Missing {
                column: col.name().to_string(),
                ty: col.dbtype().to_string(),
                nullable: col.nullable(),
            }),
        }
    }

    pub(crate) fn changed(
        schemaname: Option<&str>,
        tablename: &str,
        colcol: &(&ColumnDef, &Column),
    ) -> Self {
        let ident = TableIdent {
            schema: schemaname.map(|x| x.to_string()),
            name: tablename.to_string(),
        };
        let (db, st) = *colcol;
        let type_changed = !super::same_types(&db.ty, st.dbtype());

        let level = if !type_changed {
            Level::Medium
        } else {
            Level::High
        };

        Issue {
            ident,
            level,
            kind: Kind::Changed(Diff {
                type_changed,
                column: db.name.to_string(),
                db_type: db.ty.to_string(),
                db_nullable: db.null,
                welds_type: st.dbtype().to_string(),
                welds_nullable: st.nullable(),
            }),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Level {
    /// Full on prevents this struct from being used.
    Critical,
    /// Query might fail on edge cases of in data. (null, size overflow)
    High,
    /// Might be an issue when inserting records / updating
    Medium,
    /// Just useful to know
    Low,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Critical => "Critical".bright_red().fmt(f)?,
            Level::High => "High".red().fmt(f)?,
            Level::Medium => "Medium".yellow().fmt(f)?,
            Level::Low => "Low".blue().fmt(f)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Missing {
    pub column: String,
    pub ty: String,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// The table is missing in the database
    MissingTable,
    /// The column is defined in the database but not the welds Struct
    InDbNotModel(Missing),
    /// The column is defined on the welds Struct but not in the database
    OnModelNotDb(Missing),
    /// How the model is defined in the database table is different than on the welds Struct
    Changed(Diff),
}

impl Kind {
    pub fn as_missing_table(&self) -> Option<()> {
        match self {
            Kind::MissingTable => Some(()),
            _ => None,
        }
    }

    pub fn as_in_db_not_model(&self) -> Option<&Missing> {
        match self {
            Kind::InDbNotModel(missing) => Some(missing),
            _ => None,
        }
    }

    pub fn as_on_model_not_db(&self) -> Option<&Missing> {
        match self {
            Kind::OnModelNotDb(missing) => Some(missing),
            _ => None,
        }
    }

    pub fn as_changed(&self) -> Option<&Diff> {
        match self {
            Kind::Changed(diff) => Some(diff),
            _ => None,
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::MissingTable => {
                f.write_str("The Table was not found in the database")?;
            }
            Kind::InDbNotModel(missing) => {
                f.write_str("The Column `")?;
                f.write_str(&missing.column)?;
                f.write_str("` was defined in the database but not the struct")?;
            }
            Kind::OnModelNotDb(missing) => {
                f.write_str("The Column `")?;
                f.write_str(&missing.column)?;
                f.write_str("` was defined on the struct but not in the database")?;
            }
            Kind::Changed(diff) => diff.fmt(f)?,
        }
        Ok(())
    }
}
