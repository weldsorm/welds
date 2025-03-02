use welds_connections::Param;
pub type ParamArgs<'a> = Vec<&'a (dyn Param + Sync)>;

// Concrete Types
mod basic;
pub use basic::Basic;
mod basicopt;
pub use basicopt::BasicOpt;
mod numeric;
pub use numeric::Numeric;
mod numericopt;
pub use numericopt::NumericOpt;
mod text;
pub use text::Text;
mod textopt;
pub use textopt::TextOpt;

pub(crate) mod manualparam;

//  Relationships / SubQueries
pub(crate) mod exists;
pub(crate) mod wherein;

pub(crate) mod orderby;
pub(crate) use orderby::OrderBy;

// trait used to add where clause to a sql statement
mod clause_adder;
pub use clause_adder::ClauseAdder;

// trait used to write assignments in a sql statement
mod assignment_adder;
pub use assignment_adder::AssignmentAdder;

pub struct ClauseColVal<T> {
    pub null_clause: bool,
    pub not_clause: bool,
    pub col: String,
    pub operator: &'static str,
    pub val: Option<T>,
}

pub struct ClauseColValEqual<T> {
    pub null_clause: bool,
    pub not_clause: bool,
    pub col: String,
    pub operator: &'static str,
    pub val: Option<T>,
}

pub struct ClauseColValIn<T> {
    pub col: String,
    pub operator: &'static str,
    pub list: Vec<T>,
}

pub struct ClauseColValList<T> {
    pub col: String,
    pub operator: &'static str,
    pub list: Vec<T>,
}

pub struct ClauseColManual {
    pub(crate) col: Option<String>,
    pub(crate) sql: String,
    pub(crate) params: Vec<Box<dyn Param + Send + Sync>>,
}

pub struct AssignmentManual {
    pub(crate) col: String,
    pub(crate) sql: String,
    pub(crate) params: Vec<Box<dyn Param + Send + Sync>>,
}

// How a value should be selected out.
// colname refers to the real name of the column.
// fieldname refers to what we want to get the column out as.
// for example: select id as ids from bla.
pub trait AsFieldName<T> {
    fn colname(&self) -> &str;
    fn fieldname(&self) -> &str;
}

// marker trait to make sure a field is nullable
pub trait AsOptField {}

// Clases Used for assignment

pub struct SetColVal<T> {
    pub col_raw: String,
    pub val: T,
}

pub struct SetColNull {
    pub col_raw: String,
}
