use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn build_pool(db_url: &String) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager)
}

pub fn connect_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    build_pool(&db_url).expect("Error connecting to Postgres")
}
