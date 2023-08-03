use actix_web::web::Data;
use diesel::{r2d2::ConnectionManager, PgConnection};

use crate::utils::DATABASE_URL;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Data<Pool> {
    let db_url = DATABASE_URL.to_string();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: Pool = diesel::r2d2::Pool::builder().build(manager).expect("Failed to create DB");
    Data::new(pool.clone())
}