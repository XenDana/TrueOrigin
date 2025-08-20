use candid::Principal;
use ic_cdk::api::time;
use sha2::{Sha256, Digest};
use std::time::Duration;
use futures::channel::oneshot;
use ic_cdk_timers::set_timer;
use email_address::EmailAddress;


pub fn generate_unique_principal(principal: Principal) -> Principal {
    // Combine the principal text and the current time
    let input = format!("{}-{}", principal.to_text(), time());

    // Hash the combined input using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    // Take the first 29 bytes of the hash and convert it into a Principal
    let principal_bytes: [u8; 29] = result[0..29].try_into().expect("slice with incorrect length");

    Principal::from_slice(&principal_bytes)
}

/// Creates a future that completes after the specified duration.
/// Uses a oneshot channel and `ic_cdk_timers::set_timer`.
pub async fn async_delay(duration: Duration) {
    let (tx, rx) = oneshot::channel::<()>();
    ic_cdk::print(format!("⏱️ Setting timer for {:?}", duration)); // Optional: Log timer setting
    set_timer(duration, move || {
        let _ = tx.send(()); // Signal completion, ignore result
    });
    match rx.await {
        Ok(_) => { /* Timer completed successfully */ }
        Err(e) => {
            // This should ideally not happen in canister environment unless timer logic fails
            ic_cdk::print(format!("❌ ERROR: Timer future cancelled: {:?}", e));
        }
    }
}

/// Validates email format using RFC 5322 compliance
/// Returns true if email is valid according to email standards
/// 
/// # Examples
/// ```
/// assert!(validate_email("user@domain.com"));
/// assert!(validate_email("test+tag@example.org"));
/// assert!(!validate_email("a@."));
/// assert!(!validate_email("invalid.email"));
/// ```
pub fn validate_email(email: &str) -> bool {
    email.parse::<EmailAddress>().is_ok()
}

/// Validates string is not empty or just whitespace
pub fn validate_non_empty_string(input: &str) -> bool {
    !input.trim().is_empty()
}

/// Formats a timestamp into a human-readable string for debugging
pub fn format_timestamp(timestamp: u64) -> String {
    format!("{}ns", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid_emails() {
        // Standard email formats
        assert!(validate_email("user@domain.com"));
        assert!(validate_email("test@example.org"));
        assert!(validate_email("admin@company.net"));
        
        // Email with plus sign (common for tagging)
        assert!(validate_email("test+tag@example.org"));
        
        // Email with dots in local part
        assert!(validate_email("first.last@example.com"));
        
        // Email with numbers
        assert!(validate_email("user123@domain456.com"));
        
        // Email with hyphens in domain
        assert!(validate_email("user@my-domain.com"));
    }

    #[test]
    fn test_validate_email_invalid_emails() {
        // Empty string
        assert!(!validate_email(""));
        
        // Missing @ symbol
        assert!(!validate_email("invalid.email"));
        assert!(!validate_email("user.domain.com"));
        
        // Missing domain
        assert!(!validate_email("user@"));
        
        // Missing local part
        assert!(!validate_email("@domain.com"));
        
        // Invalid domain endings
        assert!(!validate_email("a@."));
        assert!(!validate_email("user@domain."));
        
        // Invalid characters/format
        assert!(!validate_email(".@A"));
        assert!(!validate_email("@@"));
        assert!(!validate_email("user@@domain.com"));
        
        // Just symbols
        assert!(!validate_email("@"));
        assert!(!validate_email("."));
        
        // Spaces (not allowed in email)
        assert!(!validate_email("user @domain.com"));
        assert!(!validate_email("user@ domain.com"));
    }

    #[test]
    fn test_validate_non_empty_string() {
        assert!(validate_non_empty_string("valid string"));
        assert!(validate_non_empty_string("a"));
        assert!(!validate_non_empty_string(""));
        assert!(!validate_non_empty_string("   "));
        assert!(!validate_non_empty_string("\t\n"));
    }

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(1234567890), "1234567890ns");
        assert_eq!(format_timestamp(0), "0ns");
    }
}

