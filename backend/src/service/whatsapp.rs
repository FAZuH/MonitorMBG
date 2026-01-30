//! WhatsApp Business API integration for OTP delivery.
//!
//! This module provides functionality to send OTP codes via WhatsApp
//! using the WhatsApp Business API (Meta/Facebook).

use std::sync::Arc;

use log::error;
use log::info;
use log::warn;
use serde::Deserialize;
use serde::Serialize;

use crate::config::Config;
use crate::error::AppError;

/// WhatsApp API client for sending messages.
#[derive(Clone)]
pub struct WhatsAppClient {
    config: Arc<Config>,
    http_client: reqwest::Client,
}

impl WhatsAppClient {
    /// Creates a new WhatsApp client.
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Checks if WhatsApp integration is enabled and properly configured.
    pub fn is_enabled(&self) -> bool {
        let whatsapp = &self.config.whatsapp;
        if !whatsapp.enabled {
            return false;
        }

        // Check if all required configuration is present
        whatsapp.api_url.is_some()
            && whatsapp.api_token.is_some()
            && whatsapp.phone_number_id.is_some()
    }

    /// Sends an OTP code to a phone number via WhatsApp.
    ///
    /// # Arguments
    ///
    /// * `phone` - The phone number in international format (e.g., +628123456789)
    /// * `otp_code` - The 6-digit OTP code to send
    /// * `reference_id` - A unique reference ID for this OTP request
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the message was sent successfully.
    pub async fn send_otp(
        &self,
        phone: &str,
        otp_code: &str,
        reference_id: &str,
    ) -> Result<(), AppError> {
        if !self.is_enabled() {
            warn!("WhatsApp OTP is disabled or not configured");
            return Err(AppError::ServiceUnavailable(
                "WhatsApp service is not available".to_string(),
            ));
        }

        let whatsapp = &self.config.whatsapp;
        let api_url = whatsapp.api_url.as_ref().unwrap();
        let api_token = whatsapp.api_token.as_ref().unwrap();
        let phone_number_id = whatsapp.phone_number_id.as_ref().unwrap();

        // Format phone number (ensure it starts with +)
        let formatted_phone = if phone.starts_with('+') {
            phone.to_string()
        } else if phone.starts_with('0') {
            // Convert Indonesian local format to international
            format!("+62{}", &phone[1..])
        } else {
            format!("+{}", phone)
        };

        // Build the WhatsApp API URL
        let url = format!(
            "{}/{}/messages",
            api_url.trim_end_matches('/'),
            phone_number_id
        );

        // Prepare the message payload
        let payload = WhatsAppMessageRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: formatted_phone.clone(),
            type_: "template".to_string(),
            template: Template {
                name: "monitor_mbg_otp".to_string(),
                language: Language {
                    code: "id".to_string(),
                },
                components: vec![Component {
                    type_: "body".to_string(),
                    parameters: vec![
                        Parameter {
                            type_: "text".to_string(),
                            text: otp_code.to_string(),
                        },
                        Parameter {
                            type_: "text".to_string(),
                            text: (whatsapp.otp_expiry_seconds / 60).to_string(),
                        },
                    ],
                }],
            },
        };

        // Send the request
        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send WhatsApp message: {}", e);
                AppError::ServiceUnavailable(format!("WhatsApp API error: {}", e))
            })?;

        if response.status().is_success() {
            info!(
                "OTP sent successfully to {} with reference {}",
                formatted_phone, reference_id
            );
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("WhatsApp API error: status={}, body={}", status, error_text);
            Err(AppError::ServiceUnavailable(
                "Failed to send WhatsApp message".to_string(),
            ))
        }
    }
}

// WhatsApp API request/response structures

#[derive(Serialize)]
struct WhatsAppMessageRequest {
    #[serde(rename = "messaging_product")]
    messaging_product: String,
    #[serde(rename = "recipient_type")]
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    type_: String,
    template: Template,
}

#[derive(Serialize)]
struct Template {
    name: String,
    language: Language,
    components: Vec<Component>,
}

#[derive(Serialize)]
struct Language {
    code: String,
}

#[derive(Serialize)]
struct Component {
    #[serde(rename = "type")]
    type_: String,
    parameters: Vec<Parameter>,
}

#[derive(Serialize)]
struct Parameter {
    #[serde(rename = "type")]
    type_: String,
    text: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct WhatsAppMessageResponse {
    #[serde(rename = "messages")]
    messages: Vec<Message>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Message {
    id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whatsapp_client_disabled() {
        let config = Arc::new(Config::default());
        let client = WhatsAppClient::new(config);
        assert!(!client.is_enabled());
    }
}
