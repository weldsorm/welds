use welds_connections::Syntax;

use crate::migrations::types::Type;

/// Discribes how a value is writen into SQL for a columns default value
pub trait DefaultValue {
    fn to_raw_sql(&self, syntax: Syntax, ty: &Type) -> String;
}

#[derive(Debug, Clone, Copy, Default)]
/// Use the backend "Now" function to get the current time.
pub struct DefaultValueNow {}

fn sql_string_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

impl DefaultValue for DefaultValueNow {
    fn to_raw_sql(&self, syntax: Syntax, _ty: &Type) -> String {
        match syntax {
            Syntax::Mysql => "DEFAULT CURRENT_TIMESTAMP",
            Syntax::Mssql => "DEFAULT GETDATE()",
            Syntax::Postgres => "DEFAULT now()",
            Syntax::Sqlite => "DEFAULT CURRENT_TIMESTAMP",
        }
        .to_string()
    }
}

impl DefaultValue for bool {
    fn to_raw_sql(&self, syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(&self.to_string());
                format!("DEFAULT {}", s)
            }
            _ => match syntax {
                Syntax::Mssql => {
                    // MSSQL stores bool as BIT
                    let v = if *self { "1" } else { "0" };
                    format!("DEFAULT {}", v)
                }
                _ => format!("DEFAULT {}", self),
            },
        }
    }
}

impl DefaultValue for u8 {
    fn to_raw_sql(&self, _syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(&self.to_string());
                format!("DEFAULT {}", s)
            }
            _ => format!("DEFAULT {}", self),
        }
    }
}

impl DefaultValue for i8 {
    fn to_raw_sql(&self, _syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(&self.to_string());
                format!("DEFAULT {}", s)
            }
            _ => format!("DEFAULT {}", self),
        }
    }
}
impl DefaultValue for i16 {
    fn to_raw_sql(&self, _syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(&self.to_string());
                format!("DEFAULT {}", s)
            }
            _ => format!("DEFAULT {}", self),
        }
    }
}
impl DefaultValue for i32 {
    fn to_raw_sql(&self, _syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(&self.to_string());
                format!("DEFAULT {}", s)
            }
            _ => format!("DEFAULT {}", self),
        }
    }
}
impl DefaultValue for i64 {
    fn to_raw_sql(&self, _syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(&self.to_string());
                format!("DEFAULT {}", s)
            }
            _ => format!("DEFAULT {}", self),
        }
    }
}
impl DefaultValue for i128 {
    fn to_raw_sql(&self, _syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(&self.to_string());
                format!("DEFAULT {}", s)
            }
            _ => format!("DEFAULT {}", self),
        }
    }
}

impl DefaultValue for String {
    fn to_raw_sql(&self, _syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(self);
                format!("DEFAULT {}", s)
            }
            _ => format!("DEFAULT {}", self),
        }
    }
}

impl DefaultValue for &str {
    fn to_raw_sql(&self, _syntax: Syntax, ty: &Type) -> String {
        match ty {
            Type::String | Type::StringSized(_) => {
                let s = sql_string_literal(self);
                format!("DEFAULT {}", s)
            }
            _ => format!("DEFAULT {}", self),
        }
    }
}
