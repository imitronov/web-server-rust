use crate::schema::articles;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable, Queryable, Selectable, Serialize)]
#[diesel(table_name = articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub id: i64,
    pub name: String,
    pub body: String,
    pub published: bool,
}
