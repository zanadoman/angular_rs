use core::fmt::{self, Debug, Formatter};

use axum_login::AuthUser;
use sqlx::{Error, MySqlPool, mysql::MySqlQueryResult};

#[derive(Clone, serde::Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl User {
    #[tracing::instrument(level = "trace", skip(database))]
    pub async fn find(
        database: &MySqlPool,
        name: &str,
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE name = ? LIMIT 1;",
            name
        )
        .fetch_optional(database)
        .await
    }

    #[tracing::instrument(level = "trace", skip(database))]
    pub async fn create(
        database: &MySqlPool,
        name: &str,
        password: &str,
    ) -> Result<MySqlQueryResult, Error> {
        sqlx::query!(
            "INSERT INTO users VALUES (?, ?);",
            name,
            password_auth::generate_hash(password)
        )
        .execute(database)
        .await
    }

    #[tracing::instrument(level = "trace", skip(database))]
    pub async fn validate(
        &self,
        database: &MySqlPool,
    ) -> Result<(), &'static str> {
        if self.name.is_empty() {
            return Err("Name must be at least 1 character long.");
        }
        if 50 < self.name.len() {
            return Err("Name must not be more than 50 characters long.");
        }
        match (Self::find(database, &self.name)).await {
            Ok(Some(..)) => return Err("Name already taken."),
            Err(err) => {
                tracing::error!("{err}");
                return Err("Internal server error.");
            }
            _ => {}
        }
        if self.password.len() < 8 {
            return Err("Password must be at least 8 characters long.");
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
