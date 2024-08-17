use candid::{CandidType, Nat, Principal, Deserialize};
use ic_cdk::{init, query, update};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(CandidType, Deserialize, Clone)]
struct Metadata {
    key: String,
    value: String,
}

#[derive(CandidType, Deserialize,  Clone)]
struct Organization {
    id: Principal,
    name: String,
    description: String,
    metadata: Vec<Metadata>,
    created_at: Nat,
    created_by: Principal,
    updated_at: Nat,
    updated_by: Principal,
}

#[derive(CandidType, Deserialize)]
struct OrganizationInput {
    name: String,
    description: String,
    metadata: Vec<Metadata>,
}

lazy_static! {
    static ref ORGANIZATIONS: Mutex<HashMap<Principal, Organization>> = Mutex::new(HashMap::new());
}

impl Default for Organization {
    fn default() -> Self {
        Organization {
            id: Principal::anonymous(), // Default value for Principal
            name: String::new(),
            description: String::new(),
            metadata: Vec::new(),
            created_at: Nat::from(0 as u64), // Default value for Nat
            created_by: Principal::anonymous(), // Default value for Principal
            updated_at: Nat::from(0 as u64), // Default value for Nat
            updated_by: Principal::anonymous(), // Default value for Principal
        }
    }
}

#[query]
fn getOrganizationById(id: Principal) -> Option<Organization> {
    let organizations = ORGANIZATIONS.lock().unwrap();
    organizations.get(&id).cloned()
}

#[update]
fn createOrganization(input: OrganizationInput) -> Organization {
    let id = Principal::anonymous(); // Generate a unique ID for the organization
    let organization = Organization {
        id,
        name: input.name,
        ..Default::default() 
    };
    let mut organizations = ORGANIZATIONS.lock().unwrap();
    organizations.insert(id, organization.clone());
    organization
}

#[update]
fn updateOrganization(id: Principal, input: OrganizationInput) -> Option<Organization> {
    let mut organizations = ORGANIZATIONS.lock().unwrap();
    if let Some(org) = organizations.get_mut(&id) {
        org.name = input.name;
        org.description = input.description;
        org.metadata = input.metadata;
        org.updated_at = Nat::from(0 as u64); // Update with the current timestamp
        org.updated_by = Principal::anonymous(); // Update with the current user
        Some(org.clone())
    } else {
        None
    }
}
