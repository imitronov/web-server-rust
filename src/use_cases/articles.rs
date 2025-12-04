use crate::entities::article::Article;

pub async fn create() -> Article {
    Article {
        id: 123,
        name: "Article name".to_string(),
    }
}

pub async fn index() -> Vec<Article> {
    vec![
        Article {
            id: 1,
            name: "Article name 1".to_string(),
        },
        Article {
            id: 2,
            name: "Article name 2".to_string(),
        },
        Article {
            id: 3,
            name: "Article name 3".to_string(),
        },
    ]
}

pub async fn view(id: i32) -> Article {
    Article {
        id,
        name: "Article name".to_string(),
    }
}

pub async fn put(id: i32) -> Article {
    Article {
        id,
        name: "Article name".to_string(),
    }
}

pub async fn patch(id: i32) -> Article {
    Article {
        id,
        name: "Article name".to_string(),
    }
}

pub async fn delete() {}
