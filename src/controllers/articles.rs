use crate::use_cases::articles;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, web};

pub async fn create() -> HttpResponse {
    let new_article = articles::create().await;

    HttpResponse::build(StatusCode::CREATED).json(new_article)
}

pub async fn index() -> HttpResponse {
    let articles = articles::index().await;

    HttpResponse::Ok().json(articles)
}

pub async fn view(id: web::Path<i32>) -> HttpResponse {
    let article = articles::view(id.into_inner()).await;

    HttpResponse::Ok().json(article)
}

pub async fn put(id: web::Path<i32>) -> HttpResponse {
    let article = articles::put(id.into_inner()).await;

    HttpResponse::Ok().json(article)
}

pub async fn patch(id: web::Path<i32>) -> HttpResponse {
    let article = articles::patch(id.into_inner()).await;

    HttpResponse::Ok().json(article)
}

pub async fn delete() -> HttpResponse {
    articles::delete().await;

    HttpResponse::build(StatusCode::NO_CONTENT).finish()
}
