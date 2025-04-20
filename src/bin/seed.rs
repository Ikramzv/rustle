use fake::Fake;
use fake::faker::internet::en::{FreeEmail, Username};
use fake::faker::lorem::en::{Paragraph, Sentence};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sqlx::{PgPool, postgres::PgPoolOptions};

const USER_COUNT: usize = 100;
const POST_COUNT: usize = 1000;
const COMMENT_COUNT: usize = 50;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL should be set to seed database");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    // Truncate all tables before seeding
    truncate_tables(&pool).await?;
    println!("All tables truncated successfully");

    let mut rng = StdRng::seed_from_u64(42);

    let user_ids = seed_users(&pool).await?;
    let post_ids = seed_posts(&pool, &user_ids, &mut rng).await?;
    seed_post_likes(&pool, &post_ids, &user_ids, &mut rng).await?;
    seed_post_comments(&pool, &post_ids, &user_ids, &mut rng).await?;

    println!("Database seeding completed successfully!");
    Ok(())
}

async fn truncate_tables(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Truncating all tables...");

    // Disable foreign key checks temporarily
    sqlx::query("SET session_replication_role = 'replica';")
        .execute(pool)
        .await?;

    // Truncate all tables in the correct order to avoid foreign key constraint issues
    sqlx::query("TRUNCATE TABLE post_comments CASCADE;")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE post_likes CASCADE;")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE posts_media CASCADE;")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE posts CASCADE;")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE verification_pins CASCADE;")
        .execute(pool)
        .await?;

    sqlx::query("TRUNCATE TABLE users CASCADE;")
        .execute(pool)
        .await?;

    // Re-enable foreign key checks
    sqlx::query("SET session_replication_role = 'origin';")
        .execute(pool)
        .await?;

    Ok(())
}

async fn seed_users(pool: &PgPool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("Seeding users...");

    let mut user_ids = Vec::with_capacity(USER_COUNT);

    for i in 0..USER_COUNT {
        let email: String = FreeEmail().fake();
        let username: String = Username().fake();
        let profile_image_url: Option<String> = None;

        let user_id: String = sqlx::query_scalar(
            r#"
            INSERT INTO users (email, username, profile_image_url, is_verified)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(&email)
        .bind(&username)
        .bind(&profile_image_url)
        .bind(true)
        .fetch_one(pool)
        .await?;

        user_ids.push(user_id);

        if (i + 1) % 10 == 0 {
            println!("Created {} users", i + 1);
        }
    }

    println!("Created all 100 users");
    Ok(user_ids)
}

async fn seed_posts(
    pool: &PgPool,
    user_ids: &[String],
    rng: &mut StdRng,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("Seeding posts...");

    let mut post_ids = Vec::with_capacity(POST_COUNT);

    for i in 0..POST_COUNT {
        let user_id = &user_ids[rng.random_range(0..user_ids.len())];
        let title: String = Sentence(1..3).fake();
        let content: String = Paragraph(5..20).fake();

        let post_id: String = sqlx::query_scalar(
            r#"
            INSERT INTO posts (user_id, title, content)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(user_id)
        .bind(&title)
        .bind(&content)
        .fetch_one(pool)
        .await?;

        post_ids.push(post_id);

        if (i + 1) % 100 == 0 {
            println!("Created {} posts", i + 1);
        }
    }

    println!("Created all 1000 posts");
    Ok(post_ids)
}

async fn seed_post_likes(
    pool: &PgPool,
    post_ids: &[String],
    user_ids: &[String],
    rng: &mut StdRng,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Seeding post likes...");

    let mut total_likes = 0;

    for (i, post_id) in post_ids.iter().enumerate() {
        // Generate 50-70 likes for each post
        let num_likes = rng.random_range(50..=70);

        // Create a set of unique user IDs to avoid duplicate likes
        let mut used_user_ids = std::collections::HashSet::new();

        for _ in 0..num_likes {
            // Find a user that hasn't liked this post yet
            let mut user_id;
            loop {
                user_id = &user_ids[rng.random_range(0..user_ids.len())];
                if used_user_ids.insert(user_id) {
                    break;
                }
            }

            sqlx::query(
                r#"
                INSERT INTO post_likes (post_id, user_id)
                VALUES ($1, $2)
                "#,
            )
            .bind(post_id)
            .bind(user_id)
            .execute(pool)
            .await?;

            total_likes += 1;
        }

        if (i + 1) % 100 == 0 {
            println!("Created likes for {} posts", i + 1);
        }
    }

    println!("Created {} post likes", total_likes);
    Ok(())
}

async fn seed_post_comments(
    pool: &PgPool,
    post_ids: &[String],
    user_ids: &[String],
    rng: &mut StdRng,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Seeding post comments...");

    let mut total_comments = 0;

    for (i, post_id) in post_ids.iter().enumerate() {
        // Create 50 comments for each post
        for _ in 0..COMMENT_COUNT {
            let user_id = &user_ids[rng.random_range(0..user_ids.len())];
            let content: String = Paragraph(1..5).fake();

            sqlx::query(
                r#"
                INSERT INTO post_comments (post_id, user_id, content)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(post_id)
            .bind(user_id)
            .bind(&content)
            .execute(pool)
            .await?;

            total_comments += 1;
        }

        if (i + 1) % 100 == 0 {
            println!("Created comments for {} posts", i + 1);
        }
    }

    println!("Created {} post comments", total_comments);
    Ok(())
}
