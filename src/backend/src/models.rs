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

