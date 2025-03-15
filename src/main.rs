use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::sqlite::SqlitePool;
use dotenv::dotenv;
use std::env;

mod models;
mod db;

async fn shorten(
    pool: web::Data<SqlitePool>,
    req: web::Json<models::ShortenRequest>,
) -> impl Responder {
    let short_code = db::shorten_url(&pool, &req.original_url).await;
    HttpResponse::Ok().json(models::ShortenResponse {
        short_url: format!("http://localhost:8080/{}", short_code),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/shorten", web::post().to(shorten))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
