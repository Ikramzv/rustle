use sqlx::{PgPool, Result};

use crate::{
    dtos::post::{CreatePostDto, UpdatePostDto},
    models::Post,
};

pub async fn create_post(pool: &PgPool, user_id: &str, body: CreatePostDto) -> Result<Post> {
    let post: Post = sqlx::query_as(
        r#"
        INSERT INTO posts (user_id, title, content)
        VALUES ($1, $2, $3)
        RETURNING *
    "#,
    )
    .bind(user_id)
    .bind(&body.title)
    .bind(&body.content)
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn find_posts(pool: &PgPool, offset: i32, limit: i32) -> Result<Vec<Post>> {
    let posts: Vec<Post> = sqlx::query_as(
        r#"
        SELECT * FROM posts
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        OFFSET $1
        LIMIT $2
    "#,
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(posts)
}

pub async fn find_user_posts(
    pool: &PgPool,
    user_id: &str,
    offset: i32,
    limit: i32,
) -> Result<Vec<Post>> {
    let posts: Vec<Post> = sqlx::query_as(
        r#"
        SELECT * FROM posts
        WHERE user_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        OFFSET $2
        LIMIT $3
    "#,
    )
    .bind(user_id)
    .bind(offset)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(posts)
}

pub async fn find_post_by_id(pool: &PgPool, id: &str) -> Result<Option<Post>> {
    let post: Option<Post> = sqlx::query_as(
        r#"
        SELECT * FROM posts
        WHERE id = $1 AND deleted_at IS NULL
    "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(post)
}

pub async fn update_post(
    pool: &PgPool,
    user_id: &str,
    post_id: &str,
    body: UpdatePostDto,
) -> Result<Option<Post>> {
    let updated_post: Option<Post> = sqlx::query_as(
        r#"UPDATE posts SET content = $1 WHERE user_id = $2 AND id = $3 AND deleted_at IS NULL RETURNING *"#,
    )
    .bind(body.content)
    .bind(user_id)
    .bind(post_id)
    .fetch_optional(pool)
    .await?;

    Ok(updated_post)
}

pub async fn delete_post(pool: &PgPool, user_id: &str, post_id: &str) -> Result<Option<String>> {
    let deleted_post: Option<String> = sqlx::query_scalar(
        r#"UPDATE posts SET deleted_at = NOW() where user_id = $1 AND id = $2 RETURNING id"#,
    )
    .bind(user_id)
    .bind(post_id)
    .fetch_optional(pool)
    .await?;

    Ok(deleted_post)
}
