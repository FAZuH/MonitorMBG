use std::sync::Arc;
use log::error;
use uuid::Uuid;

use crate::auth::utils::{generate_token, hash_password, verify_password};
use crate::config::Config;
use crate::database::error::DatabaseError;
use crate::database::model::{User, UserRole};
use crate::database::table::Table;
use crate::database::Database;
use crate::error::AppError;

#[derive(Clone)]
pub struct AuthService {
    db: Arc<Database>,
    config: Arc<Config>,
}

impl AuthService {
    pub fn new(db: Arc<Database>, config: Arc<Config>) -> Self {
        Self { db, config }
    }

    pub async fn register_user(
        &self,
        name: String,
        role: UserRole,
        unique_code: String,
        password: String,
        phone: Option<String>,
        institution_name: Option<String>,
    ) -> Result<(String, User), AppError> {
        let password_hash = hash_password(&password)?;

        let user = User {
            name,
            role,
            unique_code,
            password_hash: Some(password_hash),
            phone,
            institution_name,
            ..Default::default()
        };

        let user_id = self.db.user_table.insert(&user).await.map_err(|e: DatabaseError| {
            if e.to_string().contains("duplicate key") {
                AppError::BadRequest("User with this unique code already exists".to_string())
            } else {
                error!("Database error during registration: {}", e);
                AppError::InternalServerError("An unexpected error occurred".to_string())
            }
        })?;

        let created_user = self
            .db
            .user_table
            .select(&user_id)
            .await
            .map_err(|e| {
                error!("Failed to retrieve created user {}: {}", user_id, e);
                AppError::InternalServerError("Failed to retrieve created user".to_string())
            })?
            .ok_or_else(|| {
                error!("User {} not found after creation", user_id);
                AppError::InternalServerError("User not found after creation".to_string())
            })?;

        let token = generate_token(created_user.id, created_user.role, &self.config.jwt_secret)?;

        Ok((token, created_user))
    }

    pub async fn login_user(
        &self,
        unique_code: String,
        password: String,
    ) -> Result<(String, User), AppError> {
        let user = self
            .db
            .user_table
            .find_by_unique_code(&unique_code)
            .await
            .map_err(|e| {
                error!("Database error during login for user {}: {}", unique_code, e);
                AppError::InternalServerError("Database error".to_string())
            })?;

        // Mitigate User Enumeration
        let (user_found, stored_hash) = if let Some(u) = user {
            let hash = u.password_hash.clone();
            (Some(u), hash)
        } else {
            // Dummy hash for "password"
            (None, Some("$argon2id$v=19$m=4096,t=3,p=1$c2FsdHNhbHQ$aaaaaaaaaaaaaaaaaaaaaa".to_string()))
        };

        let password_valid = if let Some(hash) = stored_hash {
            match verify_password(&password, &hash) {
                Ok(valid) => valid,
                Err(_) => false,
            }
        } else {
            false
        };

        if let Some(user) = user_found {
            if password_valid {
                let token = generate_token(user.id, user.role, &self.config.jwt_secret)?;
                return Ok((token, user));
            }
        }

        Err(AppError::Unauthorized("Invalid credentials".to_string()))
    }
}
