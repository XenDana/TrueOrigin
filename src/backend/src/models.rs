use candid::{CandidType, Principal, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct Metadata {
    key: String,
    value: String,
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

#[derive(CandidType, Deserialize)]
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