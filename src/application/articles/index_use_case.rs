use crate::infrastructure::resources::ArticleResource;

pub async fn index_articles_use_case() -> Vec<ArticleResource> {
    vec![
        ArticleResource {
            id: "1".to_string(),
            name: "Article name 1".to_string(),
        },
        ArticleResource {
            id: "2".to_string(),
            name: "Article name 2".to_string(),
        },
        ArticleResource {
            id: "3".to_string(),
            name: "Article name 3".to_string(),
        },
    ]
}
