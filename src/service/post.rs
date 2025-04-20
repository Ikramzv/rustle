use std::{collections::HashMap, sync::Arc, time::Instant};

use sqlx::{PgPool, Result};

use crate::{
    dtos::post::{CreatePostDto, UpdatePostDto},
    models::{Post, PostDetails, PostMedia, User},
    service,
};

pub async fn create_post(
    pool: &PgPool,
    user_id: &str,
    body: CreatePostDto,
) -> Result<(Post, Vec<PostMedia>), String> {
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
    .await
    .map_err(|e| e.to_string())?;

    let mut handles = Vec::new();

    let pool = Arc::new(pool.clone());

    for media in body.media {
        let pool = pool.clone();
        let post_id = post.id.clone();
        let handle = tokio::spawn(async move {
            let post_media: PostMedia = sqlx::query_as(r#"
                INSERT INTO posts_media (post_id, media_url, media_type, mime_type, width, height, file_size)
                VALUES ($1, $2, $3::MediaType, $4, $5, $6, $7)
                RETURNING *
            "#)
            .bind(&post_id)
            .bind(&media.url)
            .bind(&media.r#type.to_str())
            .bind(&media.mime_type)
            .bind(&media.width)
            .bind(&media.height)
            .bind(&media.size)
            .fetch_one(&*pool)
            .await
            .map_err(|e| e.to_string())?;

            Ok::<PostMedia, String>(post_media)
        });

        handles.push(handle);
    }

    let mut post_media_list = Vec::with_capacity(handles.len());

    for handle in handles {
        post_media_list.push(handle.await.map_err(|e| e.to_string())??);
    }

    Ok((post, post_media_list))
}

pub async fn find_posts(pool: &PgPool, offset: i64, limit: i64) -> Result<Vec<PostDetails>> {
    let before = Instant::now();

    // TODO: Remove likes_count and comments_count from the query
    let posts: Vec<Post> = sqlx::query_as(
        r#"
        SELECT p.* FROM posts p
        WHERE p.deleted_at IS NULL
        ORDER BY p.created_at DESC
        OFFSET $1
        LIMIT $2
    "#,
    )
    .bind(offset)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    tracing::info!("[find_posts] Posts query time: {:?}", before.elapsed());

    if posts.is_empty() {
        return Ok(Vec::new());
    }

    // Get all media for these posts in a single query

    let post_details = get_post_details(pool, posts).await?;

    Ok(post_details)
}

pub async fn find_user_posts(
    pool: &PgPool,
    user_id: &str,
    offset: i64,
    limit: i64,
) -> Result<Vec<PostDetails>> {
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

    let post_details = get_post_details(pool, posts).await?;

    Ok(post_details)
}

pub async fn find_post_by_id(pool: &PgPool, id: &str) -> Result<Option<PostDetails>> {
    let post: Option<Post> = sqlx::query_as(
        r#"
        SELECT * FROM posts
        WHERE id = $1 AND deleted_at IS NULL
    "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match post {
        Some(post) => {
            let post_details_v = get_post_details(pool, vec![post]).await?;

            let post_details = post_details_v
                .get(0)
                .cloned()
                .expect("[find_post_by_id] post_details not found");

            Ok(Some(post_details))
        }
        None => Ok(None),
    }
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

pub async fn like_post(pool: &PgPool, user_id: &str, post_id: &str) -> Result<bool> {
    sqlx::query(r#"SELECT id FROM posts WHERE id = $1 AND deleted_at IS NULL"#)
        .bind(&post_id)
        .fetch_one(pool)
        .await?;

    let has_liked: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM post_likes
            WHERE user_id = $1 AND post_id = $2
        )
    "#,
    )
    .bind(user_id)
    .bind(post_id)
    .fetch_one(pool)
    .await?;

    let query = if has_liked {
        sqlx::query(
            r#"
            DELETE FROM post_likes
            WHERE user_id = $1 AND post_id = $2
        "#,
        )
    } else {
        sqlx::query(
            r#"
            INSERT INTO post_likes (user_id, post_id)
            VALUES ($1, $2)
        "#,
        )
    };

    query.bind(user_id).bind(post_id).execute(pool).await?;

    let has_liked = !has_liked;

    Ok(has_liked)
}

pub async fn get_post_details(pool: &PgPool, posts: Vec<Post>) -> Result<Vec<PostDetails>> {
    let post_ids: Vec<String> = posts.iter().map(|p| p.id.clone()).collect();
    let user_ids: Vec<String> = posts.iter().map(|p| p.user_id.clone()).collect();

    let (mut media_by_post, author_by_post, comments_count_by_id, likes_count_by_id) = tokio::try_join!(
        get_media_by_post_map(pool, &post_ids),
        get_author_by_id_map(pool, &user_ids),
        get_comments_count_by_id_map(pool, &post_ids),
        get_likes_count_by_id_map(pool, &post_ids),
    )?;

    // Combine posts with their media
    let post_details: Vec<PostDetails> = posts
        .into_iter()
        .map(|post| {
            let post_id = post.id.clone();
            let user_id = post.user_id.clone();

            let author = author_by_post
                .get(&user_id)
                .cloned()
                .expect("[get_post_details] author not found");

            let media = media_by_post.remove(&post_id).unwrap_or_default();

            let likes_count = likes_count_by_id.get(&post_id).cloned().unwrap_or(0);
            let comments_count = comments_count_by_id.get(&post_id).cloned().unwrap_or(0);

            PostDetails {
                post,
                author,
                media,
                likes_count,
                comments_count,
            }
        })
        .collect();

    Ok(post_details)
}

async fn get_media_by_post_map(
    pool: &PgPool,
    post_ids: &[String],
) -> Result<HashMap<String, Vec<PostMedia>>> {
    let media: Vec<PostMedia> = find_posts_media_by_post_ids(pool, post_ids).await?;

    // Group media by post_id for efficient lookup
    let mut media_by_post: HashMap<String, Vec<PostMedia>> = HashMap::new();

    for m in media {
        media_by_post.entry(m.post_id.clone()).or_default().push(m);
    }

    Ok(media_by_post)
}

async fn get_author_by_id_map(pool: &PgPool, user_ids: &[String]) -> Result<HashMap<String, User>> {
    let users: Vec<User> = service::user::get_users_by_ids(pool, &user_ids).await?;

    let user_by_id: HashMap<String, User> = users.into_iter().map(|u| (u.id.clone(), u)).collect();

    Ok(user_by_id)
}

async fn find_posts_media_by_post_ids(
    pool: &PgPool,
    post_ids: &[String],
) -> Result<Vec<PostMedia>> {
    let post_media: Vec<PostMedia> = sqlx::query_as(
        r#"
        SELECT * FROM posts_media WHERE post_id = ANY($1)
        "#,
    )
    .bind(post_ids)
    .fetch_all(pool)
    .await?;

    Ok(post_media)
}

async fn get_comments_count_by_id_map(
    pool: &PgPool,
    post_ids: &[String],
) -> Result<HashMap<String, i64>> {
    let comments_count: Vec<(String, i64)> = sqlx::query_as(
        r#"
            SELECT 
                post_id, 
                COUNT(id) as comments_count 
                FROM post_comments 
                WHERE post_id = ANY($1) 
                GROUP BY post_id
        "#,
    )
    .bind(post_ids)
    .fetch_all(pool)
    .await?;

    let comments_count_map: HashMap<String, i64> = comments_count.into_iter().collect();

    Ok(comments_count_map)
}

async fn get_likes_count_by_id_map(
    pool: &PgPool,
    post_ids: &[String],
) -> Result<HashMap<String, i64>> {
    let likes_count: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT 
            post_id, 
            COUNT(id) as likes_count 
        FROM post_likes 
        WHERE post_id = ANY($1) 
        GROUP BY post_id
        "#,
    )
    .bind(post_ids)
    .fetch_all(pool)
    .await?;

    let likes_count_map: HashMap<String, i64> = likes_count.into_iter().collect();

    Ok(likes_count_map)
}
