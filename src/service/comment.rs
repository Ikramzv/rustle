use std::collections::HashMap;

use sqlx::{PgPool, QueryBuilder, Result};

use crate::dtos::comment::{CreateCommentDto, UpdateCommentDto};
use crate::models::{PostComment, PostCommentDetails, User};
use crate::service;

pub async fn get_posts_comments(
    pool: &PgPool,
    post_id: &str,
    offset: i64,
    limit: i64,
    parent_id: Option<String>,
) -> Result<Vec<PostCommentDetails>> {
    let mut query_builder = QueryBuilder::new(
        r#"
        SELECT * FROM post_comments
        WHERE"#,
    );

    query_builder.push(" post_id = ");
    query_builder.push_bind(post_id);

    query_builder.push(" AND deleted_at IS NULL");

    match parent_id {
        Some(parent_id) => {
            query_builder.push(" AND parent_id = ");
            query_builder.push_bind(parent_id);
        }
        None => {
            query_builder.push(" AND parent_id IS NULL");
        }
    }

    query_builder.push(" ORDER BY created_at DESC");

    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);

    let query = query_builder.build_query_as();

    let comments: Vec<PostComment> = query.fetch_all(pool).await?;

    let comment_details = get_comment_details(pool, comments).await?;

    Ok(comment_details)
}

pub async fn get_user_comments(
    pool: &PgPool,
    user_id: &str,
    offset: i64,
    limit: i64,
) -> Result<Vec<PostCommentDetails>> {
    let comments: Vec<PostComment> = sqlx::query_as(
        r#"
        SELECT * FROM post_comments
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

    let comment_details = get_comment_details(pool, comments).await?;

    Ok(comment_details)
}

pub async fn create_comment(
    pool: &PgPool,
    user_id: &str,
    body: CreateCommentDto,
) -> Result<PostComment> {
    let comment: PostComment = sqlx::query_as(
        r#"
        INSERT INTO post_comments (post_id, user_id, content, parent_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&body.post_id)
    .bind(user_id)
    .bind(&body.content)
    .bind(body.parent_id)
    .fetch_one(pool)
    .await?;

    Ok(comment)
}

pub async fn update_comment(
    pool: &PgPool,
    user_id: &str,
    comment_id: &str,
    body: UpdateCommentDto,
) -> Result<PostComment> {
    let comment: PostComment = sqlx::query_as(
        r#"
        UPDATE post_comments
        SET content = $1
        WHERE id = $2 AND user_id = $3 AND deleted_at IS NULL
        RETURNING *
    "#,
    )
    .bind(&body.content)
    .bind(comment_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(comment)
}

pub async fn delete_comment(pool: &PgPool, user_id: &str, comment_id: &str) -> Result<String> {
    let comment_id: String = sqlx::query_scalar(
        r#"
        UPDATE post_comments
        SET deleted_at = NOW()
        WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL
        RETURNING id
        "#,
    )
    .bind(comment_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(comment_id)
}

async fn get_comment_details(
    pool: &PgPool,
    comments: Vec<PostComment>,
) -> Result<Vec<PostCommentDetails>> {
    let user_ids: Vec<String> = comments.iter().map(|c| c.user_id.clone()).collect();

    let users_by_id_map = get_users_by_id_map(pool, user_ids).await?;

    let comment_details = comments
        .into_iter()
        .map(|comment| {
            let author = users_by_id_map
                .get(&comment.user_id)
                .cloned()
                .expect("[get_comment_details] author not found");
            PostCommentDetails { comment, author }
        })
        .collect();

    Ok(comment_details)
}

async fn get_users_by_id_map(
    pool: &PgPool,
    user_ids: Vec<String>,
) -> Result<HashMap<String, User>> {
    let users: Vec<User> = service::user::get_users_by_ids(pool, &user_ids).await?;

    let user_by_id_map = users
        .into_iter()
        .map(|user| (user.id.clone(), user))
        .collect();

    Ok(user_by_id_map)
}
