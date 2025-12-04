use crate::use_cases::auth;
use actix_web::http::{StatusCode, header};
use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize)]
pub struct Registration {
    pub email: String,
    pub password: String,
}

pub async fn registration(data: web::Json<Registration>) -> HttpResponse {
    let result = auth::registration(data.email.to_string(), data.password.to_string());

    if result.is_ok() {
        return HttpResponse::build(StatusCode::CREATED).finish();
    }

    HttpResponse::build(StatusCode::BAD_REQUEST).json(ErrorResponse {
        message: result.err().unwrap(),
    })
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

pub async fn login(data: web::Json<Login>) -> HttpResponse {
    let result = auth::login(data.email.to_string(), data.password.to_string());

    if result.is_ok() {
        return HttpResponse::build(StatusCode::OK).json(Token {
            token: result.ok().unwrap(),
        });
    }

    HttpResponse::build(StatusCode::BAD_REQUEST).json(ErrorResponse {
        message: result.err().unwrap(),
    })
}

pub async fn whoami(req: HttpRequest) -> HttpResponse {
    let auth_string = req
        .headers()
        .get(header::AUTHORIZATION)
        .unwrap()
        .to_str()
        .unwrap();

    if auth_string.starts_with("Bearer ") {
        let token = auth_string.trim_start_matches("Bearer ");

        let user = auth::whoami(token.to_string());

        if user.is_ok() {
            return HttpResponse::build(StatusCode::OK).json(user.ok().unwrap());
        }
    }

    HttpResponse::build(StatusCode::UNAUTHORIZED).finish()
}
