use diesel::{Connection, PgConnection};

fn main() {
    println!("Hello, Diesel!");
    let mut connection = postgres_connection();
    let conn = &mut connection;

    let res = bazel_diesel_postgres::run_db_migration(conn);
    //dbg!(&result);
    assert!(res.is_ok());
}

fn postgres_connection() -> PgConnection {
    let database_url = "postgres://postgres:postgres@localhost/postgres";

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}