//! Authentication utilities for password hashing and JWT management.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use argon2::Argon2;
use argon2::password_hash::PasswordHash;
use argon2::password_hash::PasswordHasher;
use argon2::password_hash::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use jsonwebtoken::encode;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::database::model::UserRole;
use crate::error::AppError;

/// JWT claims structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID).
    pub sub: Uuid,
    /// User role.
    pub role: UserRole,
    /// Expiration timestamp.
    pub exp: usize,
    /// Issued-at timestamp.
    pub iat: usize,
}

/// Hashes a password using Argon2.
///
/// # Errors
///
/// Returns [`AppError::InternalServerError`] if hashing fails.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_token(user_id: Uuid, role: UserRole, secret: &str) -> Result<String, AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let expiration = now + 3600; // 1 hour

    let claims = Claims {
        sub: user_id,
        role,
        exp: expiration,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalServerError(e.to_string()))
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_password_hashing_success() {
        let password = "password123";
        let hash = hash_password(password).unwrap();

        // Verify the hash is not empty and different from original password
        assert!(!hash.is_empty());
        assert_ne!(hash, password);

        // Verify correct password matches
        assert!(verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_password_hashing_wrong_password() {
        let password = "password123";
        let hash = hash_password(password).unwrap();

        // Verify wrong password doesn't match
        assert!(!verify_password("wrongpassword", &hash).unwrap());
        assert!(!verify_password("", &hash).unwrap());
        assert!(!verify_password("password1234", &hash).unwrap());
    }

    #[test]
    fn test_password_hashing_different_salts() {
        let password = "password123";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        // Same password should produce different hashes due to different salts
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }

    #[test]
    fn test_password_hashing_empty_password() {
        let password = "";
        let hash = hash_password(password).unwrap();

        // Empty password should still hash and verify
        assert!(verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_password_hashing_long_password() {
        let password = "a".repeat(1000);
        let hash = hash_password(&password).unwrap();

        // Long password should still work
        assert!(verify_password(&password, &hash).unwrap());
    }

    #[test]
    fn test_jwt_token_generation_and_validation() {
        let user_id = Uuid::new_v4();
        let role = UserRole::Kitchen;
        let secret = "test_secret";

        let token = generate_token(user_id, role, secret).unwrap();
        let claims = validate_token(&token, secret).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, role);
        assert!(claims.iat > 0);
        assert!(claims.exp > claims.iat);

        // Token should expire in approximately 1 hour (3600 seconds)
        let expected_exp = claims.iat + 3600;
        assert_eq!(claims.exp, expected_exp);
    }

    #[test]
    fn test_jwt_token_different_roles() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret";

        for role in [UserRole::Admin, UserRole::Kitchen, UserRole::School] {
            let token = generate_token(user_id, role, secret).unwrap();
            let claims = validate_token(&token, secret).unwrap();
            assert_eq!(claims.role, role);
        }
    }

    #[test]
    fn test_jwt_token_invalid_secret() {
        let user_id = Uuid::new_v4();
        let role = UserRole::Kitchen;
        let secret = "test_secret";
        let wrong_secret = "wrong_secret";

        let token = generate_token(user_id, role, secret).unwrap();
        let result = validate_token(&token, wrong_secret);

        assert!(result.is_err());
        match result {
            Err(AppError::Unauthorized(_)) => (),
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[test]
    fn test_jwt_token_malformed_token() {
        let secret = "test_secret";

        let result = validate_token("not.a.valid.token", secret);
        assert!(result.is_err());

        let result = validate_token("", secret);
        assert!(result.is_err());

        let result = validate_token("invalid", secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_token_tampered() {
        let user_id = Uuid::new_v4();
        let role = UserRole::Kitchen;
        let secret = "test_secret";

        let token = generate_token(user_id, role, secret).unwrap();

        // Tamper with the token by changing a character
        let mut tampered = token.clone();
        if let Some(last) = tampered.pop() {
            tampered.push(if last == 'a' { 'b' } else { 'a' });
        }

        let result = validate_token(&tampered, secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_structure() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: Uuid::new_v4(),
            role: UserRole::Admin,
            exp: now + 3600,
            iat: now,
        };

        // Test serialization
        let json = serde_json::to_string(&claims).unwrap();
        assert!(json.contains("sub"));
        assert!(json.contains("role"));
        assert!(json.contains("exp"));
        assert!(json.contains("iat"));

        // Test deserialization
        let decoded: Claims = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.sub, claims.sub);
        assert_eq!(decoded.role, claims.role);
        assert_eq!(decoded.exp, claims.exp);
        assert_eq!(decoded.iat, claims.iat);
    }
}
