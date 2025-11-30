use crate::application::articles::index_articles_use_case;
use actix_web::HttpResponse;

pub async fn index_articles_controller() -> HttpResponse {
    let articles = index_articles_use_case().await;

    HttpResponse::Ok().json(articles)
}
