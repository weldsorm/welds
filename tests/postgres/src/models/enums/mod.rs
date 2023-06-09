// only for PostgreSQL to match a type definition

#[derive(sqlx::Type, Debug, Clone, PartialEq)]
#[sqlx(type_name = "Color")]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Default for Color {
    fn default() -> Self {
        Color::Red
    }
}
