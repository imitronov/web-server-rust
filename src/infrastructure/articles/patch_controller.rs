use crate::application::articles::patch_article_use_case;
use actix_web::{HttpResponse, web};

pub async fn patch_article_controller(id: web::Path<String>) -> HttpResponse {
    let article = patch_article_use_case(id.into_inner()).await;

    HttpResponse::Ok().json(article)
}
