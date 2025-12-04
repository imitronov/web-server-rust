use actix_web::{App, HttpServer, web};

mod controllers;
mod entities;
mod use_cases;

use crate::controllers::articles;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/articles")
                .route("", web::post().to(articles::create))
                .route("", web::get().to(articles::index))
                .route("{id}", web::get().to(articles::view))
                .route("{id}", web::put().to(articles::put))
                .route("{id}", web::patch().to(articles::patch))
                .route("{id}", web::delete().to(articles::delete)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
