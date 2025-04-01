use axum_login::{
    AuthManagerLayer, AuthManagerLayerBuilder, AuthnBackend, UserId,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
};
use sqlx::{Error, MySqlPool};
use time::Duration;

use crate::models::User;

#[derive(Clone)]
pub struct Authenticator(MySqlPool);

impl Authenticator {
    #[tracing::instrument(level = "debug", skip(pool))]
    pub fn new(
        pool: MySqlPool,
    ) -> Result<AuthManagerLayer<Self, MemoryStore>, Error> {
        Ok(AuthManagerLayerBuilder::new(
            Self(pool),
            SessionManagerLayer::new(MemoryStore::default())
                .with_expiry(Expiry::OnInactivity(Duration::days(1)))
                .with_secure(false),
        )
        .build())
    }
}

#[async_trait::async_trait]
impl AuthnBackend for Authenticator {
    type Credentials = User;
    type Error = Error;
    type User = User;

    #[tracing::instrument(level = "trace", skip(self))]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.get_user(&creds.name).await?.filter(|user| {
            password_auth::verify_password(creds.password, &user.password)
                .is_ok()
        }))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        User::find(&self.0, user_id).await
    }
}
