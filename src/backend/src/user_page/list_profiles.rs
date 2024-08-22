use ic_cdk::query;
use crate::{models::User, global_state::USERS};

#[query]
fn list_profiles() -> Result<Vec<User>, String> {
    let users = USERS.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    let profiles = users.values().cloned().collect::<Vec<_>>();
    Ok(profiles)
}
