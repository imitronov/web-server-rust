use actix_web::{App, HttpServer, web};

mod application;
mod infrastructure;

use infrastructure::articles::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/articles")
                .route("", web::post().to(create_article_controller))
                .route("", web::get().to(index_articles_controller))
                .route("{id}", web::get().to(view_article_controller))
                .route("{id}", web::put().to(put_article_controller))
                .route("{id}", web::patch().to(patch_article_controller))
                .route("{id}", web::delete().to(delete_article_controller)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
