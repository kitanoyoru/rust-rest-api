use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    sql_query,
    sqlite::SqliteConnection,
};

embed_migrations!();

pub type Connection = PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn migrate_and_configure_db(url: &str) -> Pool {
    info!("Migrating and configuring database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Faield to create pool");

    embedded_migrations::run(&pool.get().expect("Failed to migrate"));

    pool
}