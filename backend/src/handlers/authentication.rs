use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_login::AuthSession;
use sqlx::{Error, MySqlPool};

use crate::{Authenticator, models::User};

#[tracing::instrument(level = "debug", skip(database))]
#[axum::debug_handler]
pub async fn register(
    State(database): State<MySqlPool>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    if let Err(err) = user.validate(&database).await {
        (StatusCode::BAD_REQUEST, Json(err)).into_response()
    } else {
        match User::create(&database, &user.name, &user.password).await {
            Ok(..) => StatusCode::CREATED.into_response(),
            Err(Error::Database(err)) => {
                (StatusCode::CONFLICT, err.to_string()).into_response()
            }
            Err(err) => {
                tracing::error!("{err}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[tracing::instrument(level = "debug", skip(authenticator))]
#[axum::debug_handler]
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
                StatusCode::NO_CONTENT.into_response()
            }
        }
        Ok(None) => {
            (StatusCode::UNAUTHORIZED, "Invalid credential".to_string())
                .into_response()
        }
        Err(err) => {
            tracing::error!("{err}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[tracing::instrument(level = "debug", skip(authenticator))]
#[axum::debug_handler]
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
