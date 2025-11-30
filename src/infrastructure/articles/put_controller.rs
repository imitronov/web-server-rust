use crate::application::articles::put_article_use_case;
use actix_web::{HttpResponse, web};

pub async fn put_article_controller(id: web::Path<String>) -> HttpResponse {
    let article = put_article_use_case(id.into_inner()).await;

    HttpResponse::Ok().json(article)
}
