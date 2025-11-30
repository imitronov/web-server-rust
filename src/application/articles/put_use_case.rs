use crate::infrastructure::resources::ArticleResource;

pub async fn put_article_use_case(id: String) -> ArticleResource {
    ArticleResource {
        id,
        name: "Article name".to_string(),
    }
}
