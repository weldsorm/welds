pub(crate) struct OrderBy {
    pub(crate) field: String,
    pub(crate) direction: String,
}

impl OrderBy {
    pub(crate) fn new(field: impl Into<String>, dir: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: dir.into(),
        }
    }
}

pub(crate) fn to_sql(parts: &[OrderBy]) -> String {
    if parts.is_empty() {
        return "".to_owned();
    }
    let bys: Vec<String> = parts
        .iter()
        .map(|p| format!("{} {}", p.field, p.direction))
        .collect();
    let bys = bys.join(", ");
    format!("ORDER BY {}", bys)
}

#[test]
fn single_order_by_field() {
    let parts = vec![OrderBy {
        field: "f1".to_owned(),
        direction: "desc".to_owned(),
    }];
    let clause = to_sql(&parts);
    assert_eq!(clause.as_str(), "ORDER BY f1 desc")
}

#[test]
fn order_by_field_two_fields() {
    let parts = vec![
        OrderBy {
            field: "f1".to_owned(),
            direction: "desc".to_owned(),
        },
        OrderBy {
            field: "f2".to_owned(),
            direction: "asc".to_owned(),
        },
    ];
    let clause = to_sql(&parts);
    assert_eq!(clause.as_str(), "ORDER BY f1 desc, f2 asc")
}
