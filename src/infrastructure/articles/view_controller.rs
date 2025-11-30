use crate::application::articles::view_article_use_case;
use actix_web::{HttpResponse, web};

pub async fn view_article_controller(id: web::Path<String>) -> HttpResponse {
    let article = view_article_use_case(id.into_inner()).await;

    HttpResponse::Ok().json(article)
}
