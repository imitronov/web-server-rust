use actix_web::{App, HttpServer, web};

mod components;
mod controllers;
mod db;
mod entities;
mod middleware;
mod schema;
mod use_cases;

use crate::controllers::{articles, auth};
use crate::middleware::jwt_middleware::JwtMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/articles")
                    .route("", web::get().to(articles::index))
                    .route("{id}", web::get().to(articles::view))
                    .service(
                        web::scope("")
                            .route("", web::post().to(articles::create))
                            .route("{id}", web::put().to(articles::put))
                            .route("{id}", web::patch().to(articles::patch))
                            .route("{id}", web::delete().to(articles::delete))
                            .wrap(JwtMiddleware {}),
                    ),
            )
            .service(
                web::scope("/auth")
                    .route("registration", web::post().to(auth::registration))
                    .route("login", web::post().to(auth::login))
                    .service(
                        web::scope("")
                            .route("whoami", web::get().to(auth::whoami))
                            .wrap(JwtMiddleware {}),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
