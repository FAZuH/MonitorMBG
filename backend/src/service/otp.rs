//! OTP (One-Time Password) service for phone verification.
//!
//! This module provides functionality to generate, store, and verify OTP codes
//! for user phone verification. OTPs can be delivered via WhatsApp when enabled.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use log::info;
use log::warn;
use rand::Rng;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::error::AppError;
use crate::service::whatsapp::WhatsAppClient;

/// OTP entry containing the code and metadata.
#[derive(Clone, Debug)]
struct OtpEntry {
    code: String,
    phone: String,
    created_at: Instant,
    attempts: u32,
    verified: bool,
}

/// Service for managing OTP codes.
pub struct OtpService {
    config: Arc<Config>,
    whatsapp_client: Option<WhatsAppClient>,
    /// In-memory storage of active OTPs (reference_id -> OtpEntry)
    otps: RwLock<HashMap<String, OtpEntry>>,
}

impl OtpService {
    /// Creates a new OTP service.
    pub fn new(config: Arc<Config>) -> Self {
        let whatsapp_client = if config.whatsapp.enabled {
            Some(WhatsAppClient::new(config.clone()))
        } else {
            None
        };

        Self {
            config,
            whatsapp_client,
            otps: RwLock::new(HashMap::new()),
        }
    }

    /// Generates and sends an OTP code to the specified phone number.
    ///
    /// # Arguments
    ///
    /// * `phone` - The phone number to send the OTP to
    ///
    /// # Returns
    ///
    /// Returns a tuple of (reference_id, expires_in_seconds)
    pub async fn send_otp(&self, phone: String) -> Result<(String, u64), AppError> {
        // Validate phone format
        if !Self::is_valid_phone(&phone) {
            return Err(AppError::BadRequest(
                "Invalid phone number format".to_string(),
            ));
        }

        // Generate a 6-digit OTP code
        let otp_code = Self::generate_otp_code();

        // Generate a unique reference ID
        let reference_id = format!("otp_{}", uuid::Uuid::new_v4());

        // Store the OTP
        let _expiry_duration = Duration::from_secs(self.config.whatsapp.otp_expiry_seconds);
        let entry = OtpEntry {
            code: otp_code.clone(),
            phone: phone.clone(),
            created_at: Instant::now(),
            attempts: 0,
            verified: false,
        };

        {
            let mut otps = self.otps.write().await;
            otps.insert(reference_id.clone(), entry);
        }

        // Send via WhatsApp if enabled
        if let Some(ref client) = self.whatsapp_client {
            match client.send_otp(&phone, &otp_code, &reference_id).await {
                Ok(()) => {
                    info!(
                        "OTP sent via WhatsApp to {} with reference {}",
                        phone, reference_id
                    );
                }
                Err(e) => {
                    // If WhatsApp fails, we still return success but log the error
                    // In development mode, we can return the code in the response
                    warn!("Failed to send WhatsApp OTP: {}", e);

                    // If WhatsApp is not properly configured, return error
                    if !client.is_enabled() {
                        // Clean up the stored OTP
                        let mut otps = self.otps.write().await;
                        otps.remove(&reference_id);
                        return Err(AppError::ServiceUnavailable(
                            "WhatsApp service is not available".to_string(),
                        ));
                    }
                }
            }
        } else {
            warn!("WhatsApp client not available, OTP not sent");
            // In development, we might want to return the code
            #[cfg(debug_assertions)]
            {
                info!("Development mode: OTP code for {} is: {}", phone, otp_code);
            }
        }

        Ok((reference_id, self.config.whatsapp.otp_expiry_seconds))
    }

    /// Verifies an OTP code.
    ///
    /// # Arguments
    ///
    /// * `reference_id` - The reference ID from the send_otp call
    /// * `phone` - The phone number associated with the OTP
    /// * `code` - The OTP code to verify
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the OTP is valid and verified.
    pub async fn verify_otp(
        &self,
        reference_id: &str,
        phone: &str,
        code: &str,
    ) -> Result<bool, AppError> {
        let mut otps = self.otps.write().await;

        let entry = otps
            .get_mut(reference_id)
            .ok_or_else(|| AppError::NotFound("Invalid reference ID".to_string()))?;

        // Check if OTP has expired
        let expiry_duration = Duration::from_secs(self.config.whatsapp.otp_expiry_seconds);
        if entry.created_at.elapsed() > expiry_duration {
            otps.remove(reference_id);
            return Err(AppError::BadRequest("OTP has expired".to_string()));
        }

        // Check if already verified
        if entry.verified {
            return Err(AppError::BadRequest(
                "OTP has already been verified".to_string(),
            ));
        }

        // Check max attempts
        if entry.attempts >= self.config.whatsapp.max_attempts {
            otps.remove(reference_id);
            return Err(AppError::TooManyRequests(
                "Maximum verification attempts exceeded".to_string(),
            ));
        }

        // Increment attempts
        entry.attempts += 1;

        // Verify code
        if entry.code != code {
            return Ok(false);
        }

        // Verify phone matches
        let normalized_entry_phone = Self::normalize_phone(&entry.phone);
        let normalized_input_phone = Self::normalize_phone(phone);
        if normalized_entry_phone != normalized_input_phone {
            return Ok(false);
        }

        // Mark as verified
        entry.verified = true;
        info!("OTP verified successfully for reference {}", reference_id);

        Ok(true)
    }

    /// Cleans up expired OTPs. Should be called periodically.
    pub async fn cleanup_expired(&self) {
        let mut otps = self.otps.write().await;
        let expiry_duration = Duration::from_secs(self.config.whatsapp.otp_expiry_seconds);

        let expired_keys: Vec<String> = otps
            .iter()
            .filter(|(_, entry)| entry.created_at.elapsed() > expiry_duration)
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired_keys {
            otps.remove(&key);
            info!("Cleaned up expired OTP: {}", key);
        }
    }

    /// Generates a 6-digit OTP code.
    fn generate_otp_code() -> String {
        let mut rng = rand::rng();
        let code: u32 = rng.random_range(100000..=999999);
        code.to_string()
    }

    /// Validates phone number format.
    fn is_valid_phone(phone: &str) -> bool {
        // Support Indonesian formats: 08xxxxxxxx, 628xxxxxxxx, +628xxxxxxxx
        let re = regex::Regex::new(r"^(\+62|62|0)[0-9]{9,12}$").unwrap();
        re.is_match(phone)
    }

    /// Normalizes phone number for comparison.
    fn normalize_phone(phone: &str) -> String {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.starts_with('0') {
            format!("62{}", &digits[1..])
        } else {
            digits
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_otp_code() {
        let code = OtpService::generate_otp_code();
        assert_eq!(code.len(), 6);
        assert!(code.parse::<u32>().unwrap() >= 100000);
        assert!(code.parse::<u32>().unwrap() <= 999999);
    }

    #[test]
    fn test_normalize_phone() {
        assert_eq!(OtpService::normalize_phone("08123456789"), "628123456789");
        assert_eq!(OtpService::normalize_phone("628123456789"), "628123456789");
        assert_eq!(OtpService::normalize_phone("+628123456789"), "628123456789");
    }
}
