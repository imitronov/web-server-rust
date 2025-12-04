use actix_web::{App, HttpServer, web};

mod components;
mod controllers;
mod db;
mod entities;
mod schema;
mod use_cases;

use crate::controllers::{articles, auth};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/articles")
                    .route("", web::post().to(articles::create))
                    .route("", web::get().to(articles::index))
                    .route("{id}", web::get().to(articles::view))
                    .route("{id}", web::put().to(articles::put))
                    .route("{id}", web::patch().to(articles::patch))
                    .route("{id}", web::delete().to(articles::delete)),
            )
            .service(
                web::scope("/auth")
                    .route("registration", web::post().to(auth::registration))
                    .route("login", web::post().to(auth::login))
                    .route("whoami", web::get().to(auth::whoami)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
