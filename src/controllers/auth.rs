use crate::DbPool;
use crate::ErrorResponse;
use crate::use_cases::auth;
use actix_web::http::{StatusCode, header};
use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Registration {
    pub email: String,
    pub password: String,
}

pub async fn registration(pool: web::Data<DbPool>, data: web::Json<Registration>) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let result = auth::registration(
        &mut connection,
        data.email.to_string(),
        data.password.to_string(),
    );

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(message) => HttpResponse::BadRequest().json(ErrorResponse { message: &message }),
    }
}

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct Token {
    token: String,
}

pub async fn login(pool: web::Data<DbPool>, data: web::Json<Login>) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let result = auth::login(
        &mut connection,
        data.email.to_string(),
        data.password.to_string(),
    );

    match result {
        Ok(token) => HttpResponse::Ok().json(Token { token }),
        Err(message) => HttpResponse::BadRequest().json(ErrorResponse { message: &message }),
    }
}

pub async fn whoami(pool: web::Data<DbPool>, req: HttpRequest) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let auth_string = req
        .headers()
        .get(header::AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap();

    if !auth_string.starts_with("Bearer ") {
        return HttpResponse::build(StatusCode::UNAUTHORIZED).finish();
    }

    let token = auth_string.trim_start_matches("Bearer ");
    let user = auth::whoami(&mut connection, token.to_string());

    match user {
        Ok(user) => HttpResponse::build(StatusCode::OK).json(user),
        Err(message) => {
            HttpResponse::build(StatusCode::UNAUTHORIZED).json(ErrorResponse { message: &message })
        }
    }
}
