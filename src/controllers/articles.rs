use crate::use_cases::articles;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticle {
    name: String,
    body: String,
}

pub async fn create(data: web::Json<CreateArticle>) -> HttpResponse {
    let new_article = articles::create(data.name.to_string(), data.body.to_string()).await;

    HttpResponse::build(StatusCode::CREATED).json(new_article)
}

#[derive(Deserialize)]
pub struct IndexQuery {
    page: i64,
}

pub async fn index(query: web::Query<IndexQuery>) -> HttpResponse {
    let articles = articles::index(query.page).await;

    HttpResponse::Ok().json(articles)
}

pub async fn view(id: web::Path<i64>) -> HttpResponse {
    let article = articles::view(id.into_inner()).await;

    HttpResponse::Ok().json(article)
}

#[derive(Deserialize)]
pub struct UpdateArticle {
    name: String,
    body: String,
}

pub async fn put(id: web::Path<i64>, data: web::Json<UpdateArticle>) -> HttpResponse {
    let article = articles::put(
        id.into_inner(),
        data.name.to_string(),
        data.body.to_string(),
    )
    .await;

    HttpResponse::Ok().json(article)
}

#[derive(Deserialize)]
pub struct PatchArticle {
    name: Option<String>,
    body: Option<String>,
}

pub async fn patch(id: web::Path<i64>, data: web::Json<PatchArticle>) -> HttpResponse {
    let article =
        articles::patch(id.into_inner(), data.name.to_owned(), data.body.to_owned()).await;

    HttpResponse::Ok().json(article)
}

pub async fn delete(id: web::Path<i64>) -> HttpResponse {
    articles::delete(id.into_inner()).await;

    HttpResponse::build(StatusCode::NO_CONTENT).finish()
}
