use crate::components::token;
use crate::db::{establish_connection, nextval};
use crate::entities::user::User;
use crate::schema::users;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn registration(email: String, password: String) -> Result<bool, String> {
    let connection = &mut establish_connection();

    let user = users::table
        .filter(users::dsl::email.eq(email.to_lowercase()))
        .select(User::as_select())
        .first::<User>(connection);

    if user.is_ok() {
        return Err("Email already exists".to_string());
    }

    let id: i64 = nextval(connection, "users_id_seq");
    let new_user = User::new(id, email.to_lowercase(), password);

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new article");

    Ok(true)
}

pub fn login(email: String, password: String) -> Result<String, String> {
    let connection = &mut establish_connection();
    let user = users::table
        .filter(users::dsl::email.eq(email.to_lowercase()))
        .select(User::as_select())
        .first::<User>(connection)
        .expect("Could not find user");

    if user.verify_password(password) {
        return Ok(token::create(user.id));
    }

    Err("Invalid credentials.".to_string())
}

pub fn whoami(token: String) -> Result<User, String> {
    let connection = &mut establish_connection();
    let token_data = token::verify(token);
    let sub = token_data.get("sub");

    if sub.is_none() {
        return Err("Invalid credentials.".to_string());
    }

    let user = users::table
        .filter(users::dsl::id.eq(sub.unwrap().parse::<i64>().unwrap()))
        .select(User::as_select())
        .first::<User>(connection);

    if let Ok(user) = user {
        return Ok(user);
    }

    Err("Invalid credentials.".to_string())
}
