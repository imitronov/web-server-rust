use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::select;
use diesel::sql_types::BigInt;

pub fn nextval(conn: &mut PgConnection, sequence_name: &str) -> i64 {
    let query = format!("nextval('{}')", sequence_name);

    select(sql::<BigInt>(&query))
        .get_result(conn)
        .expect("Could not get next id")
}
