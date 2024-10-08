use diesel::r2d2::R2D2Connection;
use diesel::PgConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};
use std::error::Error;

mod embed_migrations;
pub mod model;
mod schema;
// use diesel::r2d2::R2D2Connection;

// Alias for a pooled database connection.
// pub type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

// Alias for a normal, single, database connection.
pub type Connection = PgConnection;


// In a Cargo-Only project, you can use the embed_migrations macro
// by uncommenting the following line:
// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// In a Bazel project, the macro does not work, so we need
// a custom migration embedding, which builds with Cargo and Bazel alike.
// Remove this if you are using Cargo only and don't need Bazel.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations::EMBEDDED_MIGRATIONS;


/// Runs all pending database migrations.
///
/// Will return an error if the database connection is invalid, or if any of the
/// migrations fail. Otherwise, it returns Ok()
///
/// # Errors
///
/// * If the database connection is invalid
/// * If checking for pending database migrations fails
/// * If any of the database migrations fail
///
pub fn run_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Check DB connection!
    match conn.ping() {
        Ok(_) => {}
        Err(e) => {
            eprint!("[run_db_migration]: Error connecting to database: {}", e);
            return Err(Box::new(e));
        }
    }
    // Run all pending migrations.
    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("[run_db_migration]: Error migrating database: {}", e);
            Err(e)
        }
    }
}


/// Revert all pending database migrations.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a `Connection` object.
///
/// # Errors
///
/// * If there is an error while connecting to the database.
/// * If there is an error while reverting the database migrations.
///
pub fn revert_db_migration(
    conn: &mut Connection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Check DB connection!
    if let Ok(_) = conn.ping() {
    } else if let Err(e) = conn.ping() {
        eprint!("[pg_cmdb]: Error connecting to database: {}", e);
        return Err(Box::new(e));
    }

    // Revert all pending migrations
    match conn.revert_all_migrations(MIGRATIONS) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprint!("[pg_cmdb]: Error reverting database migrations: {}", e);
            Err(e)
        }
    }
}
