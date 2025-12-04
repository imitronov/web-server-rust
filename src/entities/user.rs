use crate::components::password;
use crate::schema::users;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Insertable, Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    password_hash: String,
}

impl User {
    pub fn new(id: i64, email: String, password: String) -> Self {
        User {
            id,
            email,
            password_hash: password::hash(password),
        }
    }

    pub fn verify_password(&self, password: String) -> bool {
        password::verify(password, self.password_hash.to_string())
    }
}
