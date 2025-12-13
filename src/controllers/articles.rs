use crate::use_cases::articles;
use crate::{DbPool, ErrorResponse};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticle {
    name: String,
    body: String,
}

pub async fn create(pool: web::Data<DbPool>, data: web::Json<CreateArticle>) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let new_article = articles::create(
        &mut connection,
        data.name.to_string(),
        data.body.to_string(),
    )
    .await;

    match new_article {
        Ok(article) => HttpResponse::Created().json(article),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            message: "Create article failed",
        }),
    }
}

#[derive(Deserialize)]
pub struct IndexQuery {
    page: i64,
}

pub async fn index(pool: web::Data<DbPool>, query: web::Query<IndexQuery>) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let articles = articles::index(&mut connection, query.page).await;

    HttpResponse::Ok().json(articles)
}

pub async fn view(pool: web::Data<DbPool>, id: web::Path<i64>) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let article = articles::view(&mut connection, id.into_inner()).await;

    match article {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::NotFound().json(ErrorResponse {
            message: "Article not found",
        }),
    }
}

#[derive(Deserialize)]
pub struct UpdateArticle {
    name: String,
    body: String,
}

pub async fn put(
    pool: web::Data<DbPool>,
    id: web::Path<i64>,
    data: web::Json<UpdateArticle>,
) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let article = articles::put(
        &mut connection,
        id.into_inner(),
        data.name.to_string(),
        data.body.to_string(),
    )
    .await;

    match article {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::NotFound().json(ErrorResponse {
            message: "Article not found",
        }),
    }
}

#[derive(Deserialize)]
pub struct PatchArticle {
    name: Option<String>,
    body: Option<String>,
}

pub async fn patch(
    pool: web::Data<DbPool>,
    id: web::Path<i64>,
    data: web::Json<PatchArticle>,
) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let article = articles::patch(
        &mut connection,
        id.into_inner(),
        data.name.to_owned(),
        data.body.to_owned(),
    )
    .await;

    match article {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::NotFound().json(ErrorResponse {
            message: "Article not found",
        }),
    }
}

pub async fn delete(pool: web::Data<DbPool>, id: web::Path<i64>) -> HttpResponse {
    let mut connection = match pool.get() {
        Ok(connection) => connection,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                message: "DB connection error",
            });
        }
    };

    let result = articles::delete(&mut connection, id.into_inner()).await;

    match result {
        Ok(_) => HttpResponse::build(StatusCode::NO_CONTENT).finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
