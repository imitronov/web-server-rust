use crate::controllers::{articles, auth};
use crate::middleware::jwt_middleware::JwtMiddleware;
use actix_web::{App, HttpServer, web};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use serde::Serialize;

mod components;
mod controllers;
mod entities;
mod middleware;
mod schema;
mod use_cases;

#[derive(Serialize)]
struct ErrorResponse<'a> {
    message: &'a str,
}

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
