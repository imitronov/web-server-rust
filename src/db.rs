use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::select;
use diesel::sql_types::BigInt;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn nextval(conn: &mut PgConnection, sequence_name: &str) -> i64 {
    let query = format!("nextval('{}')", sequence_name);

    select(sql::<BigInt>(&query))
        .get_result(conn)
        .expect("Could not get next id")
}
