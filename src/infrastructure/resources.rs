use serde::Serialize;

#[derive(Serialize)]
pub struct ArticleResource {
    pub id: String,
    pub name: String,
}
