use sqlx::{PgPool, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL should be set to seed database");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    seed_users(&pool).await;
    seed_posts(&pool).await;
    seed_post_likes(&pool).await;
    seed_post_comments(&pool).await;

    Ok(())
}

async fn seed_users(pool: &PgPool) {
    println!("Seeding users...");
}

async fn seed_posts(pool: &PgPool) {
    println!("Seeding posts...");
}

async fn seed_post_likes(pool: &PgPool) {
    println!("Seeding post likes...");
}

async fn seed_post_comments(pool: &PgPool) {
    println!("Seeding post comments...");
}
