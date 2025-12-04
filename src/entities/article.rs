use serde::Serialize;

#[derive(Serialize)]
pub struct Article {
    pub id: i32,
    pub name: String,
}
