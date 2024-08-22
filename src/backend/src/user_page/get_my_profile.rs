use ic_cdk::query;

use crate::{models::{User,GetMyProfileResponse}, global_state::USERS};

/// Returns the profile of the caller if it exists.
#[query]
fn get_my_profile() -> GetMyProfileResponse {
    let caller = ic_cdk::caller();
    let users: std::sync::MutexGuard<std::collections::HashMap<candid::Principal, User>> = USERS.lock().map_err(|_| GetMyProfileResponse::Err("Failed to acquire lock".to_string())).unwrap();
    match users.get(&caller) {
        Some(user) => {
            ic_cdk::print(user.id.to_text());
            GetMyProfileResponse::Ok(user.clone())
        },
        None => GetMyProfileResponse::Err("No profile found for the given address".to_string()),
    }
}
