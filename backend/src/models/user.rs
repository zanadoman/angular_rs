use core::fmt::{self, Debug, Formatter};

use axum_login::AuthUser;
use sqlx::{Error, MySqlPool};

#[derive(Clone, serde::Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl User {
    #[tracing::instrument(level = "trace", skip(pool))]
    pub async fn create(
        pool: &MySqlPool,
        name: &str,
        password: &str,
    ) -> Result<String, Error> {
        sqlx::query!(
            "INSERT INTO users VALUES (?, ?);",
            name,
            password_auth::generate_hash(password)
        )
        .execute(pool)
        .await?;
        Ok(name.to_owned())
    }

    #[tracing::instrument(level = "trace", skip(pool))]
    pub async fn find(
        pool: &MySqlPool,
        name: &str,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE name = ? LIMIT 1;",
            name
        )
        .fetch_optional(pool)
        .await
    }

    #[tracing::instrument(level = "trace", skip(pool))]
    pub async fn validate(
        &self,
        pool: &MySqlPool,
    ) -> Result<(), Option<&'static str>> {
        if self.name.is_empty() {
            return Err(Some("Name must be at least 1 character long."));
        }
        if 50 < self.name.len() {
            return Err(Some("Name must not be more than 50 characters long."));
        }
        match (Self::find(pool, &self.name)).await {
            Ok(Some(..)) => return Err(Some("Name already taken.")),
            Err(err) => {
                tracing::error!("{err}");
                return Err(None);
            }
            _ => {}
        }
        if self.password.len() < 8 {
            return Err(Some("Password must be at least 8 characters long."));
        }
        Ok(())
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("password", &"********")
            .finish()
    }
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.name.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}
