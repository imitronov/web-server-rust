use crate::infrastructure::resources::ArticleResource;

pub async fn create_article_use_case() -> ArticleResource {
    ArticleResource {
        id: "123".to_string(),
        name: "Article name".to_string(),
    }
}
