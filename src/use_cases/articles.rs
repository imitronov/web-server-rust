use crate::components::db::nextval;
use crate::entities::article::Article;
use crate::schema::articles;
use diesel::prelude::*;

pub async fn create(
    connection: &mut PgConnection,
    name: String,
    body: String,
) -> QueryResult<Article> {
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
}

pub async fn index(connection: &mut PgConnection, page: i64) -> Vec<Article> {
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

pub async fn view(connection: &mut PgConnection, id: i64) -> QueryResult<Article> {
    articles::dsl::articles
        .filter(articles::dsl::id.eq(id))
        .limit(1)
        .select(Article::as_select())
        .first(connection)
}

pub async fn put(
    connection: &mut PgConnection,
    id: i64,
    name: String,
    body: String,
) -> QueryResult<Article> {
    diesel::update(articles::dsl::articles.filter(articles::dsl::id.eq(id)))
        .set((articles::dsl::name.eq(name), articles::dsl::body.eq(body)))
        .returning(Article::as_returning())
        .get_result(connection)
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = articles)]
pub struct UpdateArticle {
    pub name: Option<String>,
    pub body: Option<String>,
}

pub async fn patch(
    connection: &mut PgConnection,
    id: i64,
    name: Option<String>,
    body: Option<String>,
) -> QueryResult<Article> {
    let update_data = UpdateArticle { name, body };

    if update_data.name.is_none() && update_data.body.is_none() {
        return view(connection, id).await;
    }

    diesel::update(articles::dsl::articles.filter(articles::dsl::id.eq(id)))
        .set(&update_data)
        .returning(Article::as_returning())
        .get_result(connection)
}

pub async fn delete(connection: &mut PgConnection, id: i64) -> QueryResult<usize> {
    diesel::delete(articles::dsl::articles.filter(articles::dsl::id.eq(id))).execute(connection)
}
