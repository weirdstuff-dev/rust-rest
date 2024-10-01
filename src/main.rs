use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = models::establish_connection(&database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::create_brand)
            .service(handlers::get_brand)
            .service(handlers::get_all_brands)
            .service(handlers::update_brand)
            .service(handlers::delete_brand)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}