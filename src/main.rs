use actix_web::{App, HttpServer};
use db::create_pool;
use dotenvy::dotenv;
use utils::API_PORT;

pub mod db;
pub mod handlers;
pub mod utils;
pub mod routes;
pub mod models;
pub mod schema;
pub mod external;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_pool();
    let server = HttpServer::new(move || App::new().app_data(pool.clone()));
    server
        .bind(("0.0.0.0", *API_PORT))
        .expect(&format!(
            "Could not bind server to port {}. Maybe port is taken ?",
            *API_PORT
        ))
        .run()
        .await
}
