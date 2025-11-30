use actix_web::HttpResponse;
use actix_web::http::StatusCode;

use crate::application::articles::*;

pub async fn create_article_controller() -> HttpResponse {
    let new_article = create_article_use_case().await;

    HttpResponse::build(StatusCode::CREATED).json(new_article)
}
