#[derive(Debug, Clone)]
pub struct Order {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<f64>,
}
