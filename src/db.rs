use sqlx::{Pool, Sqlite};
use rand::{distributions::Alphanumeric, Rng};

pub async fn shorten_url(pool: &Pool<Sqlite>, original_url: &str) -> String {
    let short_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    sqlx::query("INSERT INTO urls (short_code, original_url) VALUES (?, ?)")
        .bind(&short_code)
        .bind(original_url)
        .execute(pool)
        .await
        .unwrap();

    short_code
}
