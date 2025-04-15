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

    pub(crate) fn write(&self, table_alias: &str) -> String {
        if self.manual {
            self.field.replace("$", table_alias)
        } else {
            format!("{}.{} {}", table_alias, self.field, self.direction)
        }
    }
}

pub(crate) fn to_sql(parts: &[OrderBy], table_alias: &str) -> String {
    if parts.is_empty() {
        return "".to_owned();
    }
    let bys: Vec<String> = parts
        .iter()
        .map(|order_by| order_by.write(table_alias))
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
    let clause = to_sql(&parts, "t1");
    assert_eq!(clause.as_str(), "ORDER BY t1.f1 desc")
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
    let clause = to_sql(&parts, "t33");
    assert_eq!(clause.as_str(), "ORDER BY t33.f1 desc, t33.f2 asc")
}
