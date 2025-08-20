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
    match validate_email_with_reason(email) {
        Ok(_) => true,
        Err(err) => {
            // Only log in canister environment (not during tests)
            #[cfg(not(test))]
            ic_cdk::print(format!("Email validation failed for '{}': {}", email, err));
            false
        }
    }
}

/// Validates email format and returns detailed error information
/// Returns Ok(()) if valid, Err(String) with reason if invalid
/// 
/// # Examples
/// ```
/// assert!(validate_email_with_reason("user@domain.com").is_ok());
/// assert!(validate_email_with_reason("test+tag@example.org").is_ok());
/// 
/// let result = validate_email_with_reason("a@.");
/// assert!(result.is_err());
/// assert!(result.unwrap_err().contains("domain"));
/// ```
pub fn validate_email_with_reason(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    
    if email.len() > 254 {
        return Err("Email too long (max 254 characters)".to_string());
    }
    
    match email.parse::<EmailAddress>() {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Invalid email format: {}", err))
    }
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
    fn test_validate_email_with_reason_valid_emails() {
        // Valid emails should return Ok(())
        assert!(validate_email_with_reason("user@domain.com").is_ok());
        assert!(validate_email_with_reason("test+tag@example.org").is_ok());
        assert!(validate_email_with_reason("first.last@example.com").is_ok());
        assert!(validate_email_with_reason("user123@domain456.com").is_ok());
    }

    #[test]
    fn test_validate_email_with_reason_invalid_emails() {
        // Empty email
        let result = validate_email_with_reason("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Email cannot be empty");
        
        // Too long email (over 254 characters)
        let long_email = format!("{}@domain.com", "a".repeat(250));
        let result = validate_email_with_reason(&long_email);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Email too long"));
        
        // Invalid format - missing @
        let result = validate_email_with_reason("invalid.email");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid email format"));
        
        // Invalid format - missing domain
        let result = validate_email_with_reason("user@");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid email format"));
        
        // Invalid format - malformed domain
        let result = validate_email_with_reason("user@.");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid email format"));
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

