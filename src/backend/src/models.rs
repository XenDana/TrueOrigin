use std::fmt;

use ic_cdk::api;
use candid::{CandidType, Principal, Deserialize};

use crate::error::GenericError;

#[derive(CandidType, Deserialize, Clone)]
pub struct Metadata {
    key: String,
    value: String,
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Meta [{}: {}]", self.key, self.value)
    }
}

#[derive(CandidType, Deserialize,  Clone)]
pub struct Organization {
    pub id: Principal,
    pub name: String,
    pub description: String,
    pub metadata: Vec<Metadata>,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: u64,
    pub updated_by: Principal,
}

impl Default for Organization {
    fn default() -> Self {
        Organization {
            id: Principal::anonymous(), // Default value for Principal
            name: String::new(),
            description: String::new(),
            metadata: Vec::new(),
            created_at: api::time(),
            created_by: api::caller(), // Default value for Principal
            updated_at: api::time(),
            updated_by: api::caller(), // Default value for Principal
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct OrganizationInput {
    pub name: String,
    pub description: String,
    pub metadata: Vec<Metadata>,
}

#[derive(CandidType, Deserialize,  Clone)]
pub struct Product {
    pub id: Principal,
    pub name: String,
    pub org_id: Principal,
    pub category: String,
    pub description: String,
    pub metadata: Vec<Metadata>,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: u64,
    pub updated_by: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct ProductInput {
    pub name: String,
    pub org_id: Principal,
    pub category: String,
    pub description: String,
    pub metadata: Vec<Metadata>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct User {
    pub id: Principal,
    pub is_principal: bool,
    pub is_enabled: bool,
    pub org_ids: Vec<Principal>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_no: Option<String>,
    pub email: Option<String>,
    pub detail_meta: Vec<Metadata>,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: u64,
    pub updated_by: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct UserDetailsInput {
    pub first_name: String,
    pub last_name: String,
    pub phone_no: String,
    pub email: String,
    pub detail_meta: Vec<Metadata>,
}

#[derive(CandidType, Deserialize)]
pub enum UserResult {
    User(Option<User>),
    Err(GenericError)
}