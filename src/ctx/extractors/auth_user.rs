// use axum::extract::FromRequestParts;

// use crate::{config::CONFIG, ctx::utils::jwt, models::error::HttpError};

#[derive(Clone)]
pub struct AuthUser(pub String);

// impl<S> FromRequestParts<S> for AuthUser
// where
//     S: Sync + 'static,
// {
//     type Rejection = HttpError;

//     async fn from_request_parts(
//         parts: &mut axum::http::request::Parts,
//         _: &S,
//     ) -> Result<Self, Self::Rejection> {
//         let header = parts
//             .headers
//             .get("Authorization")
//             .and_then(|header| header.to_str().ok());

//         if header.is_none() {
//             return Err(HttpError::unauthorized(
//                 "No authorization header found".to_string(),
//             ));
//         }

//         let header = header.unwrap();

//         let token = header
//             .split(" ")
//             .nth(1)
//             .ok_or(HttpError::unauthorized("No token found".to_string()))?;

//         let user_id = jwt::validate_token(token, &CONFIG.jwt_secret)
//             .await
//             .map_err(|e| HttpError::unauthorized(e.to_string()))?;

//         Ok(AuthUser(user_id))
//     }
// }
