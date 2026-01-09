use welds_connections::Syntax;

use crate::writers::ColumnWriter;

#[derive(Clone)]
pub struct OrderBy {
    pub(crate) field: String,
    pub(crate) direction: String,
    pub(crate) manual: bool,
}

impl OrderBy {
    pub(crate) fn new(field: impl Into<String>, dir: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: dir.into(),
            manual: false,
        }
    }

    pub(crate) fn new_manual(field: impl Into<String>, dir: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: dir.into(),
            manual: true,
        }
    }

    pub(crate) fn write(&self, syntax: Syntax, table_alias: &str) -> String {
        let col_writer = ColumnWriter::new(syntax);
        if self.manual {
            self.field.replace("$", table_alias)
        } else {
            format!(
                "{}.{} {}",
                table_alias,
                col_writer.excape(&self.field),
                self.direction
            )
        }
    }
}

pub(crate) fn to_sql(syntax: Syntax, parts: &[OrderBy], table_alias: &str) -> String {
    if parts.is_empty() {
        return "".to_owned();
    }
    let bys: Vec<String> = parts
        .iter()
        .map(|order_by| order_by.write(syntax, table_alias))
        .collect();
    let bys = bys.join(", ");
    format!("ORDER BY {}", bys)
}

#[test]
fn single_order_by_field() {
    let parts = vec![OrderBy {
        field: "f1".to_owned(),
        direction: "desc".to_owned(),
        manual: false,
    }];
    let clause = to_sql(Syntax::Sqlite, &parts, "t1");
    assert_eq!(clause.as_str(), r#"ORDER BY t1."f1" desc"#)
}

#[test]
fn order_by_field_two_fields() {
    let parts = vec![
        OrderBy {
            field: "f1".to_owned(),
            direction: "desc".to_owned(),
            manual: false,
        },
        OrderBy {
            field: "f2".to_owned(),
            direction: "asc".to_owned(),
            manual: false,
        },
    ];
    let clause = to_sql(Syntax::Sqlite, &parts, "t33");
    assert_eq!(clause.as_str(), r#"ORDER BY t33."f1" desc, t33."f2" asc"#)
}
