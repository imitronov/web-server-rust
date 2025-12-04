use crate::db::{establish_connection, nextval};
use crate::entities::article::Article;
use crate::schema::articles;
use diesel::prelude::*;

pub async fn create(name: String, body: String) -> Article {
    let connection = &mut establish_connection();
    let id: i64 = nextval(connection, "articles_id_seq");

    let new_article = Article {
        id,
        name,
        body,
        published: true,
    };

    diesel::insert_into(articles::table)
        .values(&new_article)
        .returning(Article::as_returning())
        .get_result(connection)
        .expect("Error saving new article")
}

pub async fn index(page: i64) -> Vec<Article> {
    let connection = &mut establish_connection();
    let page_size: i64 = 5;
    let skip = (page - 1) * page_size;

    let query = articles::dsl::articles
        .filter(articles::dsl::published.eq(true))
        .order(articles::dsl::id.asc())
        .limit(page_size);

    QueryDsl::offset(query, skip)
        .select(Article::as_select())
        .load(connection)
        .expect("Error loading posts")
}

pub async fn view(id: i64) -> Article {
    let connection = &mut establish_connection();

    articles::dsl::articles
        .filter(articles::dsl::id.eq(id))
        .limit(1)
        .select(Article::as_select())
        .first(connection)
        .expect("Error loading posts")
}

pub async fn put(id: i64, name: String, body: String) -> Article {
    let connection = &mut establish_connection();

    diesel::update(articles::dsl::articles.filter(articles::dsl::id.eq(id)))
        .set((articles::dsl::name.eq(name), articles::dsl::body.eq(body)))
        .returning(Article::as_returning())
        .get_result(connection)
        .expect("Error updating article")
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = articles)]
pub struct UpdateArticle {
    pub name: Option<String>,
    pub body: Option<String>,
}

pub async fn patch(id: i64, name: Option<String>, body: Option<String>) -> Article {
    let connection = &mut establish_connection();

    let update_data = UpdateArticle { name, body };

    if update_data.name.is_none() && update_data.body.is_none() {
        return view(id).await;
    }

    diesel::update(articles::dsl::articles.filter(articles::dsl::id.eq(id)))
        .set(&update_data)
        .returning(Article::as_returning())
        .get_result(connection)
        .expect("Error updating article")
}

pub async fn delete(id: i64) {
    let connection = &mut establish_connection();

    diesel::delete(articles::dsl::articles.filter(articles::dsl::id.eq(id)))
        .execute(connection)
        .expect("Error deleting row");
}
