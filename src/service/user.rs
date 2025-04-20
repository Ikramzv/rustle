use sqlx::{PgPool, Result};

use crate::{dtos::user::UpdateProfileDto, models::User};

pub async fn create_user_if_not_exists(
    pool: &PgPool,
    email: &str,
    username: &str,
    profile_image_url: Option<String>,
) -> Result<User> {
    let user: Option<User> = get_user_by_email(pool, email).await?;

    if let Some(user) = user {
        return Ok(user);
    }

    let user: User = sqlx::query_as(
        r#"INSERT INTO users 
            (email, username, profile_image_url)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
    )
    .bind(email)
    .bind(username)
    .bind(profile_image_url)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: &str) -> Result<Option<User>> {
    let user: Option<User> = sqlx::query_as(r#"SELECT * FROM users WHERE id = $1"#)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn get_users_by_ids(pool: &PgPool, user_ids: &[String]) -> Result<Vec<User>> {
    let users: Vec<User> = sqlx::query_as(r#"SELECT * FROM users WHERE id = ANY($1)"#)
        .bind(user_ids)
        .fetch_all(pool)
        .await?;

    Ok(users)
}
pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    let user: Option<User> = sqlx::query_as(r#"SELECT * FROM users WHERE username = $1"#)
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    let user: Option<User> = sqlx::query_as(r#"SELECT * FROM users WHERE email = $1"#)
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn update_user(pool: &PgPool, user_id: &str, payload: UpdateProfileDto) -> Result<User> {
    let user = match get_user_by_id(pool, user_id).await? {
        Some(user) => user,
        None => return Err(sqlx::Error::RowNotFound),
    };

    let profile_image_url = match payload.profile_image_url {
        Some(url) => Some(url),
        None => user.profile_image_url,
    };

    let user: User = sqlx::query_as(
        r#"UPDATE users SET username = $1, profile_image_url = $2 WHERE id = $3 RETURNING *"#,
    )
    .bind(payload.username.unwrap_or(user.username))
    .bind(profile_image_url)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}
