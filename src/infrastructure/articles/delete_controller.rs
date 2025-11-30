use crate::application::articles::delete_article_use_case;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;

pub async fn delete_article_controller() -> HttpResponse {
    delete_article_use_case().await;

    HttpResponse::build(StatusCode::NO_CONTENT).finish()
}
