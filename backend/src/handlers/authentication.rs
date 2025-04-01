use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_login::AuthSession;
use sqlx::{Error, MySqlPool};

use crate::{Authenticator, models::User};

#[axum::debug_handler]
#[tracing::instrument(level = "debug", skip(pool))]
pub async fn register(
    State(pool): State<MySqlPool>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    if let Err(err) = user.validate(&pool).await {
        (StatusCode::BAD_REQUEST, Json(err)).into_response()
    } else {
        match User::create(&pool, &user.name, &user.password).await {
            Ok(..) => (StatusCode::CREATED, Json(user.name)).into_response(),
            Err(Error::Database(err)) => {
                (StatusCode::CONFLICT, Json(err.to_string())).into_response()
            }
            Err(err) => {
                tracing::error!("{err}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[axum::debug_handler]
#[tracing::instrument(level = "debug", skip(authenticator))]
pub async fn login(
    mut authenticator: AuthSession<Authenticator>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    match authenticator.authenticate(user).await {
        Ok(Some(user)) => {
            if let Err(err) = authenticator.login(&user).await {
                tracing::error!("{err}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            } else {
                (StatusCode::OK, Json(user.name)).into_response()
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, Json("Invalid credentials."))
            .into_response(),
        Err(err) => {
            tracing::error!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[axum::debug_handler]
#[tracing::instrument(level = "debug", skip(authenticator))]
pub async fn logout(
    mut authenticator: AuthSession<Authenticator>,
) -> impl IntoResponse {
    if let Err(err) = authenticator.logout().await {
        tracing::error!("{err}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        StatusCode::NO_CONTENT.into_response()
    }
}
