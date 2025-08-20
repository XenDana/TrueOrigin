// Application constants for TrueOrigin backend
// This file contains configurable constants used throughout the application

// Validation limits
pub const MAX_METADATA_ITEMS: usize = 50;
pub const MAX_ORGANIZATION_NAME_LENGTH: usize = 100;
pub const MAX_PRODUCT_NAME_LENGTH: usize = 100;
pub const MAX_DESCRIPTION_LENGTH: usize = 1000;
pub const MIN_NAME_LENGTH: usize = 3;

// Pagination defaults
pub const DEFAULT_PAGINATION_LIMIT: u32 = 10;
pub const MAX_PAGINATION_LIMIT: u32 = 100;

// Rate limiting
pub const MAX_VERIFICATION_ATTEMPTS_PER_HOUR: u32 = 10;
pub const RATE_LIMIT_WINDOW_SECONDS: u64 = 3600; // 1 hour

// Reward system
pub const REWARD_EXPIRATION_HOURS: u64 = 24;
pub const FIRST_VERIFICATION_REWARD_POINTS: u32 = 100;
pub const MULTIPLE_VERIFICATION_PENALTY_POINTS: u32 = 0;

// Cryptographic constants
pub const ECDSA_KEY_LENGTH: usize = 64;
pub const UNIQUE_CODE_LENGTH: usize = 32;

// API versioning
pub const API_VERSION: &str = "1.0";
pub const CANISTER_VERSION: &str = "1.0.0";